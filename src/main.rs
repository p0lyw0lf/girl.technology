use axum::{
    debug_handler,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use clap::Parser;
use std::net::Ipv4Addr;
use std::sync::OnceLock;
use tera::Tera;

fn tera() -> &'static Tera {
    static TERA: OnceLock<Tera> = OnceLock::new();
    TERA.get_or_init(|| Tera::new("templates/**/*.html").unwrap())
}

async fn index() -> Html<String> {
    Html(
        tera()
            .render("index.html", &tera::Context::new())
            .unwrap(),
    )
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

    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();

    for name in tera().get_template_names() {
        println!("{name}");
    }

    println!("Now listening at http://{bind_addr}");
    axum::serve(listener, app).await.unwrap();
}
