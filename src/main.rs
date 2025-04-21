use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::response::Redirect;
use axum::routing::get;
use axum::Router;
use diesel::prelude::*;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use tera::Context;
use tera::Tera;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod checker;
mod models;
mod schema;

#[cfg(feature = "admin")]
mod admin;

#[cfg(feature = "static")]
mod r#static;

type Rejection = (StatusCode, String);

#[derive(Clone, FromRef)]
struct AppState {
    tera: Arc<Tera>,
    default_context: Context,
    pool: Pool<AsyncPgConnection>,
}

async fn index(
    State(AppState {
        tera,
        default_context,
        ..
    }): State<AppState>,
) -> Html<String> {
    Html(tera.render("index.html.jinja", &default_context).unwrap())
}

fn render(tera: &Tera, template: &str, context: &Context) -> Result<Html<String>, Rejection> {
    Ok(Html(
        tera.render(template, &context).map_err(internal_error)?,
    ))
}

async fn list(
    State(AppState {
        tera,
        default_context,
        pool,
    }): State<AppState>,
) -> Result<Html<String>, Rejection> {
    use self::schema::listings::dsl::*;

    let conn = &mut pool.get().await.map_err(internal_error)?;
    let all_listings: Vec<_> = listings
        .select(models::Listing::as_select())
        .order_by(timestamp)
        .load(conn)
        .await
        .expect("Error loading listing");

    let mut context = default_context.clone();
    context.insert("listings", &all_listings);

    render(&tera, "list.html.jinja", &context)
}

async fn category(
    State(AppState { pool, .. }): State<AppState>,
    Path(input_category): Path<String>,
) -> Result<Redirect, Rejection> {
    use self::schema::listings::dsl::*;

    let conn = &mut pool.get().await.map_err(internal_error)?;
    let results = diesel::QueryDsl::filter(listings, category.eq(&input_category))
        .select(models::Listing::as_select())
        .load(conn)
        .await
        .expect("Error loading listing");

    let listing = results
        .get(0)
        .ok_or_else(|| external_error(format!("listing for {input_category} not found")))?;

    Ok(Redirect::temporary(&listing.url))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                [
                    "info",
                    "tower_http::trace::make_span=debug",
                    "tower_http::trace::on_request=debug",
                    "tower_http::trace::on_response=debug",
                    "girl_technology=debug",
                ]
                .join(",")
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let bind_addr = {
        let ip = env::var("IP").unwrap_or(Ipv4Addr::LOCALHOST.to_string());
        let port = env::var("PORT").unwrap_or(3000.to_string());
        format!("{ip}:{port}")
    };

    let (is_local, main_domain) = match env::var("MAIN_DOMAIN") {
        Ok(main_domain) => (false, main_domain),
        Err(_) => (true, bind_addr.clone()),
    };

    let scheme = if is_local { "http" } else { "https" };

    let tera = {
        let mut tera = Tera::new("templates/**/*.html.jinja").expect("Template parsing error");
        tera.autoescape_on(vec![".html.jinja"]);
        tera.register_filter("category_to_url", {
            let main_domain = main_domain.clone();
            move |category: &tera::Value, _: &_| {
                let category = match category.as_str() {
                    Some(category) => category,
                    None => return Err(tera::Error::msg("invalid category")),
                };
                // TODO: other validation, just in case ?
                let url = if is_local {
                    format!("{scheme}://{main_domain}/category/{category}")
                } else {
                    format!("{scheme}://{category}.{main_domain}/")
                };
                Ok(url.into())
            }
        });
        tera
    };

    let default_context = {
        let mut context = tera::Context::new();
        context.insert("main_url", &format!("{scheme}://{main_domain}/"));
        context
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(database_config)
        .build()
        .expect("Could not build postgres connection pool");

    let app = Router::new()
        .route("/", get(index))
        .route("/list", get(list))
        .route("/category/:category", get(category))
        .nest("/new", crate::checker::routes());

    #[cfg(feature = "admin")]
    let app = app.nest("/admin", crate::admin::routes());

    let app = app.with_state(AppState {
        tera: Arc::new(tera),
        default_context,
        pool,
    });

    #[cfg(feature = "static")]
    let app = app.merge(crate::r#static::routes());

    let app = app.layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> Rejection
where
    E: ToString,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}

/// Utility function for mapping any error into a `400 Bad Request` response.
fn external_error<E>(err: E) -> Rejection
where
    E: ToString,
{
    (StatusCode::BAD_REQUEST, err.to_string())
}
