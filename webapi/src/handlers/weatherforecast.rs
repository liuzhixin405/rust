
use crate::models::weatherforecast::WeatherForecast;
use actix_web::{get,HttpResponse,Responder};
use chrono::{Duration, Utc};
use rand::Rng;

#[get("/getweatherforecast")]
pub async fn getweatherforecast()->impl Responder{
    let mut rng = rand::thread_rng();
    let summaries: Vec<&str> = vec!["Sunny","Cloudy","Rainy","Stormy"];
    let weather_forecasts:Vec<WeatherForecast> = (1..=5).map(|index|{
        let date = Utc::now().date_naive() + Duration::days(index);
        let temperature_c = rng.gen_range(-20..=55);
        let summary = summaries[rng.gen_range(0..summaries.len())].to_string();
        let temperature_f =  32 + (temperature_c / 5 * 9);
        WeatherForecast{
            date,
            temperature_c,
            temperature_f,
            summary:summary
        }
    }).collect();
    HttpResponse::Ok().json(weather_forecasts)
}