use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::Serialize;

#[derive(Serialize)]
struct Property {
    id : u32,
    name : String,
    location : String
}

async fn get_properties() -> impl Responder {

    let properties = vec![
        Property { id : 1, name : "Villa".to_string(), location : "Beachside".to_string()},
        Property { id : 2, name : "Apartment".to_string(), location : "Downtown".to_string()}
    ];

    HttpResponse::Ok().json(properties)

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

        HttpServer::new(|| {
            App::new().route("/properties", web::get().to(get_properties))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

