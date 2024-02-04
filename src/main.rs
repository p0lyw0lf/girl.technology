use std::env;
use std::net::Ipv4Addr;
use std::sync::Arc;

use axum::async_trait;
use axum::extract::FromRef;
use axum::extract::FromRequestParts;
use axum::extract::State;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use clap::Parser;
use diesel::prelude::*;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use dotenvy::dotenv;
use tera::Context;
use tera::Tera;

mod models;
mod schema;

#[cfg(feature = "admin")]
mod admin;

type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

#[derive(Clone, FromRef)]
struct AppState {
    tera: Arc<Tera>,
    default_context: Context,
    pool: Pool,
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
    DatabaseConnection(mut connection): DatabaseConnection,
    State(AppState {
        tera,
        default_context,
        ..
    }): State<AppState>,
) -> Html<String> {
    use self::schema::listings::dsl::*;

    let results = listings
        .select(models::Listing::as_select())
        .load(&mut connection)
        .await
        .expect("Error loading listing");

    let mut context = default_context.clone();
    context.insert("listings", &results);

    Html(tera.render("list.html.jinja", &context).unwrap())
}

#[derive(Parser)]
struct Cli {
    ip: Option<Ipv4Addr>,
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dotenv().ok();

    let tera = {
        let mut tera = Tera::new("templates/**/*.html.jinja").expect("Template parsing error");
        tera.autoescape_on(vec![".html.jinja"]);
        tera
    };

    let bind_addr = {
        let ip = cli.ip.unwrap_or(Ipv4Addr::LOCALHOST);
        let port = cli.port.unwrap_or(3000);
        format!("{ip}:{port}")
    };

    let default_context = {
        let mut context = tera::Context::new();
        let main_url = env::var("MAIN_URL").unwrap_or_else(|_| format!("http://{bind_addr}"));
        context.insert("main_url", &main_url);
        context
    };

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_config =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = bb8::Pool::builder()
        .build(database_config)
        .await
        .expect("Could not build postgres connection pool");

    let app = Router::new()
        .route("/", get(index))
        .route("/list", get(list));

    #[cfg(feature = "admin")]
    let app = crate::admin::register(app);

    let app = app.with_state(AppState {
        tera: Arc::new(tera),
        default_context,
        pool,
    });

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}

struct DatabaseConnection(
    bb8::PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>,
);

#[async_trait]
impl<S> FromRequestParts<S> for DatabaseConnection
where
    S: Send + Sync,
    Pool: FromRef<S>,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool.get_owned().await.map_err(internal_error)?;

        Ok(Self(conn))
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
