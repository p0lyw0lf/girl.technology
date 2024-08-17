use diesel::deserialize::FromSql;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::Queryable;
use diesel::deserialize::{self};
use diesel::expression::AsExpression;
use diesel::expression::Selectable;
use diesel::pg::Pg;
use diesel::pg::PgValue;
use diesel::serialize::Output;
use diesel::serialize::ToSql;
use diesel::serialize::{self};
use diesel::sql_types::Timestamp;
use diesel::sql_types::Timestamptz;
use diesel::sql_types::{self};
use serde::Deserialize;
use serde::Serialize;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = crate::schema::listings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Listing {
    pub category: String,
    pub url: String,
    pub timestamp: PgTimestamp,
}

// Copying this from the diesel source code because it doesn't support serde

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    AsExpression,
    FromSqlRow,
    Serialize,
    Deserialize,
)]
#[diesel(sql_type = Timestamp)]
#[diesel(sql_type = Timestamptz)]
/// Timestamps are represented in Postgres as a 64 bit signed integer representing the number of
/// microseconds since January 1st 2000. This struct is a dumb wrapper type, meant only to indicate
/// the integer's meaning.
pub struct PgTimestamp(pub i64);

impl ToSql<sql_types::Timestamp, Pg> for PgTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::BigInt, Pg>::to_sql(&self.0, out)
    }
}

impl FromSql<sql_types::Timestamp, Pg> for PgTimestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<sql_types::BigInt, Pg>::from_sql(bytes).map(PgTimestamp)
    }
}

impl ToSql<sql_types::Timestamptz, Pg> for PgTimestamp {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        ToSql::<sql_types::Timestamp, Pg>::to_sql(self, out)
    }
}

impl FromSql<sql_types::Timestamptz, Pg> for PgTimestamp {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        FromSql::<sql_types::Timestamp, Pg>::from_sql(bytes)
    }
}
