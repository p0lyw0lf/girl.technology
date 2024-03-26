use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;

use axum::extract::FromRef;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use clap::Parser;
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

async fn list(
    State(AppState {
        tera,
        default_context,
        pool,
    }): State<AppState>,
) -> Result<Html<String>, Rejection> {
    use diesel::dsl::count;

    use self::schema::listings::dsl::*;

    let conn = &mut pool.get().await.map_err(internal_error)?;
    let categories: Vec<String> = listings
        .select(category)
        .group_by(category)
        .order_by(count(category))
        .load(conn)
        .await
        .expect("Error loading listing");

    let mut context = default_context.clone();
    context.insert("categories", &categories);

    Ok(Html(
        tera.render("list.html.jinja", &context)
            .map_err(internal_error)?,
    ))
}

async fn category(
    State(AppState {
        tera,
        default_context,
        pool,
    }): State<AppState>,
    Path(input_category): Path<String>,
) -> Result<Html<String>, Rejection> {
    use self::schema::listings::dsl::*;

    let conn = &mut pool.get().await.map_err(internal_error)?;
    let results = diesel::QueryDsl::filter(listings, category.eq(&input_category))
        .select(models::Listing::as_select())
        .load(conn)
        .await
        .expect("Error loading listing");

    let mut context = default_context.clone();
    context.insert("listings", &results);
    context.insert("category", &input_category);

    Ok(Html(
        tera.render("category.html.jinja", &context)
            .map_err(internal_error)?,
    ))
}

#[derive(Parser)]
struct Cli {
    ip: Option<Ipv4Addr>,
    port: Option<u16>,
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

    let cli = Cli::parse();
    dotenv().ok();

    let bind_addr = {
        let ip = cli.ip.unwrap_or(Ipv4Addr::LOCALHOST);
        let port = cli.port.unwrap_or(3000);
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
                    format!("{scheme}://{main_domain}/{category}")
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
        .route("/:category", get(category));

    #[cfg(feature = "admin")]
    let app = crate::admin::register(app);

    #[cfg(feature = "static")]
    let app = crate::r#static::register(app);

    let app = app
        .with_state(AppState {
            tera: Arc::new(tera),
            default_context,
            pool,
        })
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> Rejection
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
