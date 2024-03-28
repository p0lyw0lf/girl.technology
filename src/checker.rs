use axum::debug_handler;
use axum::extract::State;
use axum::response::Html;
use axum::routing::post;
use axum::Form;
use axum::Router;
use diesel::AsChangeset;
use diesel::Insertable;
use diesel_async::RunQueryDsl;
use regex::Regex;
use serde::Deserialize;

use url::Url;

use crate::internal_error;
use crate::AppState;
use crate::Rejection;

/// Checks that a given URL is valid, and returns the normalized version of it.
fn parse_url(url: String) -> Result<Url, String> {
    let url = match Url::parse(&url) {
        Ok(url) => url,
        Err(e) => return Err(format!("error parsing url: {e}")),
    };

    match url.scheme() {
        "http" | "https" => {}
        scheme => return Err(format!("invalid url scheme {scheme}")),
    };

    Ok(url)
}

/// Checks that a given URL has a .well-known/girl.technology file, and returns the contents of
/// that file if it exists. Otherwise, returns a description of what went wrong.
async fn check_url(mut url: Url) -> Result<String, String> {
    // Manipulate in a separate context, so that we drop the `mut path` before `url`
    {
        let mut path = match url.path_segments_mut() {
            Ok(path) => path,
            Err(e) => return Err(format!("url doesn't have a path: {e:?}")),
        };

        path.extend([".well-known", "girl.technology"]);
    }

    // TODO: use a proper reqwest client, with timeouts and maximum content lengths and everything
    let res = match reqwest::get(url.to_string()).await {
        Ok(res) => res,
        Err(e) => return Err(format!("error fetching url: {e}")),
    };

    // TODO: also check error code, we only want to parse 200s
    let text = match res.text().await {
        Ok(text) => text,
        Err(e) => return Err(format!("error getting request body: {e}")),
    };

    Ok(text)
}

lazy_static::lazy_static! {
    static ref CATEGORY_REGEX: Regex = Regex::new("^[a-z0-9-]{1,255}$").unwrap();
}

/// Checks that a given category is valid, returning the final verified category if so, and
/// returning an error message if not.
fn validate_category(category: String) -> Result<String, &'static str> {
    let category = category.trim();
    if category.len() > 255 {
        return Err("category is too long, max 255 chars");
    }
    if category.len() == 0 {
        return Err("category is 0 characters");
    }

    let bytes = category.as_bytes();
    if bytes[0] == b'-' || bytes[bytes.len() - 1] == b'-' {
        return Err("category cannot start or end with -");
    }

    if !CATEGORY_REGEX.is_match(category) {
        return Err("category must only contain 0-9, a-z, and -");
    }

    Ok(category.to_string())
}

async fn get_new(
    State(AppState {
        tera,
        default_context,
        ..
    }): State<AppState>,
) -> Result<Html<String>, Rejection> {
    crate::render(&tera, "new.html.jinja", &default_context)
}

#[derive(AsChangeset, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::listings)]
pub struct NewListing {
    pub category: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct NewListingForm {
    pub url: String,
}

#[debug_handler]
async fn post_new(
    State(AppState {
        tera,
        default_context,
        pool,
    }): State<AppState>,
    Form(NewListingForm { url }): Form<NewListingForm>,
) -> Result<Html<String>, Rejection> {
    let render_message = move |message: &str| {
        let mut context = default_context.clone();
        context.insert("message", message);
        crate::render(&tera, "new.html.jinja", &context)
    };

    let url = match parse_url(url) {
        Ok(url) => url,
        Err(e) => return render_message(&e),
    };

    let category = match check_url(url.clone()).await {
        Ok(category) => category,
        Err(e) => return render_message(&e),
    };
    let category = match validate_category(category) {
        Ok(category) => category,
        Err(e) => return render_message(e),
    };

    let success_message = format!("successfully added to the {category} category!");

    use crate::schema::listings;

    let conn = &mut pool.get().await.map_err(internal_error)?;
    let values = NewListing {
        category,
        url: url.to_string(),
    };

    diesel::insert_into(listings::table)
        .values(&values)
        .on_conflict(listings::url)
        .do_update()
        .set(&values)
        .execute(conn)
        .await
        .map_err(|_| internal_error("error saving listing"))?;

    render_message(&success_message)
}

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/",
        post(post_new)
            // TODO: make all these constant values configurable somehow
            .layer(tower::layer::util::Stack::new(
                tower::limit::RateLimitLayer::new(100, core::time::Duration::from_secs(1)),
                tower::buffer::BufferLayer::new(100),
            ))
            .route_layer(tower::timeout::TimeoutLayer::new(
                core::time::Duration::from_secs(30),
            ))
            .handle_error(|err: tower::BoxError| async { crate::internal_error(err) })
            // Don't apply ratelimiting to `get`, only to `post`
            .get(get_new),
    )
}
