// @generated automatically by Diesel CLI.

diesel::table! {
    agents (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        contact_info -> Nullable<Text>,
    }
}

diesel::table! {
    properties (id) {
        id -> Int4,
        name -> Varchar,
        location -> Varchar,
        agent_id -> Nullable<Int4>,
        #[max_length = 255]
        property_for -> Nullable<Varchar>,
        short_description -> Nullable<Text>,
        long_description -> Nullable<Text>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        whose_property -> Nullable<Varchar>,
    }
}

diesel::table! {
    propertycategories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    propertycategoriesjoin (property_id, category_id) {
        property_id -> Int4,
        category_id -> Int4,
    }
}

diesel::table! {
    propertytypes (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
    }
}

diesel::table! {
    propertytypesjoin (property_id, type_id) {
        property_id -> Int4,
        type_id -> Int4,
    }
}

diesel::joinable!(properties -> agents (agent_id));
diesel::joinable!(propertycategoriesjoin -> properties (property_id));
diesel::joinable!(propertycategoriesjoin -> propertycategories (category_id));
diesel::joinable!(propertytypesjoin -> properties (property_id));
diesel::joinable!(propertytypesjoin -> propertytypes (type_id));

diesel::allow_tables_to_appear_in_same_query!(
    agents,
    properties,
    propertycategories,
    propertycategoriesjoin,
    propertytypes,
    propertytypesjoin,
);
