use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Form;
use axum::Router;
use diesel_async::RunQueryDsl;

use crate::checker::NewListing;
use crate::internal_error;
use crate::AppState;
use crate::Rejection;

async fn submit(
    State(AppState {
        tera,
        default_context,
        ..
    }): State<AppState>,
) -> Result<Html<String>, Rejection> {
    crate::render(&tera, "admin_submit.html.jinja", &default_context)
}

async fn submit_post(
    app_state: State<AppState>,
    Form(new_listing): Form<NewListing>,
) -> Result<Html<String>, Rejection> {
    use crate::schema::listings;

    let conn = &mut app_state.pool.get().await.map_err(internal_error)?;

    diesel::insert_into(listings::table)
        .values(&new_listing)
        .execute(conn)
        .await
        .expect("Error saving listing");

    submit(app_state).await
}

pub fn routes() -> Router<AppState> {
    Router::new().route("/submit", get(submit).post(submit_post))
}
