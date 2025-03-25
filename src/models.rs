use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::{agents, properties, propertycategories, propertycategoriesjoin, propertytypes, propertytypesjoin};

// Model for the `agents` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = agents)]
pub struct Agent {
    pub id: i32,
    pub name: String,
    pub contact_info: Option<String>,
}

// Model for the `properties` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = properties)]
pub struct Property {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub agent_id: Option<i32>,
    pub property_for: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub whose_property: Option<String>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = properties)]
pub struct NewProperty {
    pub name: String,
    pub location: String,
    pub agent_id: Option<i32>,
    pub property_for: Option<String>,
    pub short_description: Option<String>,
    pub long_description: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub city: Option<String>,
    pub whose_property: Option<String>,
}


// Model for the `propertycategories` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = propertycategories)]
pub struct PropertyCategory {
    pub id: i32,
    pub name: String,
}

// Model for the `propertycategoriesjoin` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = propertycategoriesjoin)]
pub struct PropertyCategoryJoin {
    pub property_id: i32,
    pub category_id: i32,
}

// Model for the `propertytypes` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = propertytypes)]
pub struct PropertyType {
    pub id: i32,
    pub name: String,
}

// Model for the `propertytypesjoin` table
#[derive(Queryable, Serialize, Deserialize, Insertable)]
#[diesel(table_name = propertytypesjoin)]
pub struct PropertyTypeJoin {
    pub property_id: i32,
    pub type_id: i32,
}
 