use axum::Router;
use tower_http::services::ServeDir;

use crate::AppState;

pub fn register(app: Router<AppState>) -> Router<AppState> {
    // See the esbuild project in the `static` folder
    app.nest_service("/static/dist", ServeDir::new("static/dist"))
        .nest_service("/assets", ServeDir::new("assets"))
}
