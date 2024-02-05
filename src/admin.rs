use axum::extract::State;
use axum::response::Html;
use axum::routing::get;
use axum::Form;
use axum::Router;
use diesel::prelude::*;
use diesel::Insertable;
use diesel_async::RunQueryDsl;
use serde::Deserialize;

use crate::internal_error;
use crate::models::Listing;
use crate::AppState;
use crate::Rejection;

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::listings)]
pub struct NewListing {
    pub category: String,
    pub url: String,
}

async fn submit(
    State(AppState {
        tera,
        default_context,
        ..
    }): State<AppState>,
) -> Result<Html<String>, Rejection> {
    Ok(Html(
        tera.render("submit.html.jinja", &default_context)
            .map_err(internal_error)?,
    ))
}

async fn submit_post(
    app_state: State<AppState>,
    Form(new_listing): Form<NewListing>,
) -> Result<Html<String>, Rejection> {
    use crate::schema::listings;

    let conn = &mut app_state.pool.get().await.map_err(internal_error)?;

    diesel::insert_into(listings::table)
        .values(&new_listing)
        .returning(Listing::as_returning())
        .get_result(conn)
        .await
        .expect("Error saving listing");

    submit(app_state).await
}

pub fn register(app: Router<AppState>) -> Router<AppState> {
    app.route("/submit", get(submit).post(submit_post))
}
