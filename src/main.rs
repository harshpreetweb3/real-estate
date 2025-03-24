//use modules
mod schema;
mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
// use serde::Serialize;
use actix_cors::Cors;

//for connection establishment
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

//for crud operations
use crate::schema::properties::dsl::*;
use crate::models::{Property, NewProperty};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}


// #[derive(Serialize)]
// struct Property {
//     id : u32,
//     name : String,
//     location : String
// }

async fn get_properties() -> impl Responder {

    // let properties = vec![
    //     Property { id : 1, name : "Villa".to_string(), location : "Beachside".to_string()},
    //     Property { id : 2, name : "Apartment".to_string(), location : "Downtown".to_string()}
    // ];

    let mut connection = establish_connection();
    let results = properties.load::<Property>(&mut connection).expect("Error loading properties");
    HttpResponse::Ok().json(results)
}

async fn create_property(new_property : web::Json<NewProperty>) -> impl Responder{
    let mut connection = establish_connection();

    let inserted_property : Property = diesel::insert_into(properties)
    .values(&*new_property)
    .get_result(&mut connection)
    .expect("Error inserting new property");
    
    HttpResponse::Ok().json(inserted_property)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

        HttpServer::new(|| {

            let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

            App::new()
            .wrap(cors)
            .route("/properties", web::get().to(get_properties))
            .route("/properties", web::post().to(create_property))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}