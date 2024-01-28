use axum::{response::Html, routing::get, Form, Router};
use diesel::prelude::*;
use diesel::Insertable;
use serde::Deserialize;

use crate::{models::Listing, pg, DEFAULT_CONTEXT, TERA};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::listings)]
pub struct NewListing {
    pub category: String,
    pub url: String,
}

async fn submit() -> Html<String> {
    Html(TERA.render("submit.html.jinja", &DEFAULT_CONTEXT).unwrap())
}

async fn submit_post(Form(new_listing): Form<NewListing>) -> Html<String> {
    use crate::schema::listings;

    let conn = &mut pg();

    diesel::insert_into(listings::table)
        .values(&new_listing)
        .returning(Listing::as_returning())
        .get_result(conn)
        .expect("Error saving listing");

    submit().await
}

pub fn register(app: Router) -> Router {
    app.route("/submit", get(submit).post(submit_post))
}
