mod countries;
mod selector;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

use serde::Serialize;

use countries::extract_countries;

#[derive(Serialize)]
struct Country {
    name: String,
    capital: String,
}

#[get("/country/{name}")]
async fn get_country(name: web::Path<String>) -> impl Responder {
    let parsed_response = extract_countries().await;
    let (country, capital) = parsed_response.get_key_value(&name.to_string()).unwrap();
    HttpResponse::Ok().json(web::Json(Country {
        name: country.to_string(),
        capital: capital.to_string(),
    }))
}

#[get("/")]
async fn scraper() -> impl Responder {
    let parsed_response = extract_countries().await;
    let mut countries: Vec<Country> = vec![];
    for (country, capital) in parsed_response.iter() {
        countries.push(Country {
            name: country.to_string(),
            capital: capital.to_string(),
        })
    }
    HttpResponse::Ok().json(countries)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(scraper).service(get_country))
        .bind("127.0.0.1:8080")?
        .workers(4) // turn this into a multi-thread server
        .run()
        .await
}
