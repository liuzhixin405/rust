
use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct WeatherForecast{  
    pub date:NaiveDate,
    pub temperature_c:i32,
    pub temperature_f:i32,
    pub summary:String
}