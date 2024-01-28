use diesel::{data_types::PgTimestamp, deserialize::Queryable, expression::Selectable};

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::listings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Listing {
    pub id: i32,
    pub category: String,
    pub url: String,
    pub timestamp: PgTimestamp,
}
