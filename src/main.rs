use axum::{response::Html, routing::get, Router};
use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, net::Ipv4Addr};
use tera::Tera;

mod models;
mod schema;

#[cfg(feature = "admin")]
mod admin;

lazy_static::lazy_static! {
    static ref TERA: Tera = {
        let mut tera = Tera::new("templates/**/*.html.jinja").expect("Template parsing error");
        tera.autoescape_on(vec![".html.jinja"]);
        tera
    };
    static ref BIND_ADDR: String = {
        let cli = Cli::parse();
        let ip = cli.ip.unwrap_or(Ipv4Addr::LOCALHOST);
        let port = cli.port.unwrap_or(3000);
        format!("{ip}:{port}")
    };
    static ref DEFAULT_CONTEXT: tera::Context = {
        let mut context = tera::Context::new();
        dotenv().ok();
        let main_url = env::var("MAIN_URL").unwrap_or_else(|_| format!("http://{}", BIND_ADDR.to_string()));
        context.insert("main_url", &main_url);
        context
    };
    static ref DATABASE_URL: String = {
        dotenv().ok();
        env::var("DATABASE_URL").expect("DATABASE_URL must be set")
    };
}

/** Not static, since it should be created fresh each time I think */
fn pg() -> PgConnection {
    PgConnection::establish(&DATABASE_URL)
        .unwrap_or_else(|_| panic!("Error connecting to {}", DATABASE_URL.to_string()))
}

async fn index() -> Html<String> {
    Html(TERA.render("index.html.jinja", &DEFAULT_CONTEXT).unwrap())
}

async fn list() -> Html<String> {
    use self::schema::listings::dsl::*;

    let connection = &mut pg();

    let results = listings
        .select(models::Listing::as_select())
        .load(connection)
        .expect("Error loading listing");

    let mut context = DEFAULT_CONTEXT.clone();
    context.insert("listings", &results);

    Html(TERA.render("list.html.jinja", &context).unwrap())
}

#[derive(Parser)]
struct Cli {
    ip: Option<Ipv4Addr>,
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/list", get(list));

    #[cfg(feature = "admin")]
    let app = crate::admin::register(app);

    let bind_addr = BIND_ADDR.to_string();
    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
