use diesel::prelude::*;
use diesel::pg::PgConnection;
use serde::{Serialize, Deserialize};
use crate::schema::properties;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Property {
    pub id: i32,
    pub name: String,
    pub location: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = properties)]
pub struct NewProperty {
    pub name: String,
    pub location: String,
}
