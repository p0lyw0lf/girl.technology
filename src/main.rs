use axum::{response::Html, routing::get, Router};
use clap::Parser;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::net::Ipv4Addr;
use std::sync::OnceLock;
use tera::Tera;

mod models;
mod schema;

fn tera() -> &'static Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| Tera::new("templates/**/*.html").unwrap())
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

async fn index() -> Html<String> {
    Html(tera().render("index.html", &tera::Context::new()).unwrap())
}

async fn list() -> String {
    use self::schema::listings::dsl::*;

    let connection = &mut establish_connection();

    let results = listings
        .select(models::Listing::as_select())
        .load(connection)
        .expect("Error loading listing");

    results
        .into_iter()
        .map(|result| format!("{:?}", result))
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(Parser)]
struct Cli {
    ip: Option<Ipv4Addr>,
    port: Option<u16>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let ip = cli.ip.unwrap_or(Ipv4Addr::LOCALHOST);
    let port = cli.port.unwrap_or(3000);
    let bind_addr = format!("{ip}:{port}");

    let app = Router::new()
        .route("/", get(index))
        .route("/list", get(list));

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
