#[path = "models/mod.rs"]
mod models;
#[path = "handlers/mod.rs"]
mod handlers;

use actix_web::{App,HttpServer};
use handlers::*;

#[actix_web::main]
async fn main()->std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .service(weatherforecast::getweatherforecast)
    }).bind("127.0.0.1:8088")?.run().await
}


