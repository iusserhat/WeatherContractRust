xtern crate reqwest;

struct WeatherContract {
    city: String,
    country_code: String,
    api_key: String,
}

impl WeatherContract {
    fn new(city: &str, country_code: &str, api_key: &str) -> WeatherContract {
        WeatherContract {
            city: city.to_string(),
            country_code: country_code.to_string(),
            api_key: api_key.to_string(),
        }
    }

    fn get_weather(&self) -> Result<(), Box<dyn std::error::Error>> {
        let api_endpoint = "http://api.openweathermap.org/data/2.5/weather?q=";
        let full_url = format!(
            "{}{},{}&APPID={}",
            api_endpoint, self.city, self.country_code, self.api_key
        );

        let response = reqwest::blocking::get(&full_url)?;
        if !response.status().is_success() {
            return Err(format!("Failed to fetch weather data. Status code: {}", response.status()).into());
        }

        let weather_data: serde_json::Value = response.json()?;
        
        if let Some(temp) = weather_data["main"]["temp"].as_f64() {
            println!("Temperature: {}", temp);
        } else {
            return Err("Temperature data not found in the response.".into());
        }

        if let Some(pressure) = weather_data["main"]["pressure"].as_f64() {
            println!("Pressure: {}", pressure);
        } else {
            return Err("Pressure data not found in the response.".into());
        }

        if let Some(humidity) = weather_data["main"]["humidity"].as_f64() {
            println!("Humidity: {}", humidity);
        } else {
            return Err("Humidity data not found in the response.".into());
        }

        if let Some(wind_speed) = weather_data["wind"]["speed"].as_f64() {
            println!("Wind Speed: {}", wind_speed);
        } else {
            return Err("Wind speed data not found in the response.".into());
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contract = WeatherContract::new("Ankara", "TR", "your_openweathermap_api_key");
    contract.get_weather()?;
    Ok(())
}
