mod countries;
mod selector;

use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};

use countries::{extract, Country};
use dotenv::dotenv;
use lazy_static::lazy_static;
use shuttle_actix_web::ShuttleActixWeb;
use std::env;
lazy_static! {
    static ref PORT: String = {
        dotenv().ok(); // Load .env file if using dotenv
        env::var("PORT").expect("PORT must be set")
    };
}

#[get("/country/{name}")]
async fn get_country(name: web::Path<String>) -> impl Responder {
    let parsed_response = extract().await;
    let (country, capital) = parsed_response.get_key_value(&name.to_string()).unwrap();
    HttpResponse::Ok().json(web::Json(Country {
        name: country.to_string(),
        capital: capital.to_string(),
    }))
}

#[get("/")]
async fn scraper() -> impl Responder {
    let parsed_response = extract().await;
    let mut countries: Vec<Country> = vec![];
    for (country, capital) in parsed_response.iter() {
        countries.push(Country {
            name: country.to_string(),
            capital: capital.to_string(),
        })
    }
    HttpResponse::Ok().json(countries)
}
#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(scraper).service(get_country);
    };

    Ok(config.into())
}
