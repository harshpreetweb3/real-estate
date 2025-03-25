//use modules
mod schema;
mod models;

use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;

//for connection establishment
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;

//for APIs
use crate::schema::properties::dsl::*;
use crate::models::{Property, NewProperty};


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

async fn get_properties() -> impl Responder {
    let mut connection = establish_connection();
    match properties.load::<Property>(&mut connection) {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching properties"),
    }
}

// async fn create_property(new_property: web::Json<Property>) -> impl Responder {
//     let mut connection = establish_connection();
//     match diesel::insert_into(properties)
//         .values(&*new_property)
//         .get_result::<Property>(&mut connection) {
//             Ok(inserted_property) => HttpResponse::Ok().json(inserted_property),
//             Err(_) => HttpResponse::InternalServerError().body("Error inserting property"),
//     }
// }

async fn create_property(new_property: web::Json<NewProperty>) -> impl Responder {
    let mut connection = establish_connection();
    match diesel::insert_into(properties)
        .values(&*new_property)
        .get_result::<Property>(&mut connection) 
    {
        Ok(inserted_property) => HttpResponse::Ok().json(inserted_property),
        Err(e) => {
            eprintln!("Database insert error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error inserting property: {:?}", e))
        }
    }
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