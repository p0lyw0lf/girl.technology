use axum::Router;
use tower_http::services::ServeDir;

pub fn routes() -> Router {
    // See the esbuild project in the `static` folder
    Router::new()
        .nest_service("/static", ServeDir::new("static/dist"))
        .nest_service("/assets", ServeDir::new("assets"))
}
