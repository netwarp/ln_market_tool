use std::env;
use std::str::FromStr;
use serde_json::{Value};
use tokio::fs;
use reqwest;
use reqwest::Response;


pub async fn get_conf_file() -> Result<toml::Value, Box<dyn std::error::Error>> {

    let args: Vec<String> = env::args().collect();

    let conf_arg: Option<&String> = args.iter().find(|&arg| arg.starts_with("conf="));

    if let Some(conf_arg) = conf_arg {

        let conf_path: String = conf_arg.trim_start_matches("conf=").to_string();
        println!("{}", conf_path);

        let toml_content = match fs::read_to_string(&conf_path).await {
            Ok(content) => content,
            Err(err) => return Err(Box::new(err))
        };

        let parsed_toml = toml::de::from_str(&toml_content)?;

        return Ok(parsed_toml);
    }
    Err("argument --conf not found".into())
}

pub async fn get_last_ticker() -> Result<Value, Box<dyn std::error::Error>> {
    let url = "https://api.lnmarkets.com/v2/futures/ticker";
    let response: Response = reqwest::get(url).await?;
    let body = response.text().await?;
    let json_data: Value = serde_json::from_str(&body)?;
    Ok(json_data)
}

pub fn get_last_price_from_ticker(json_data: &Value) -> Option<f64> {
    match json_data["lastPrice"].as_f64() {
        Some(last_price) => Some(last_price),
        None => None,
    }
}

pub fn get_variation_from_prices(vec_prices: Vec<f64>) -> f64 {
    let first: &String = &vec_prices.first().unwrap().to_string();
    let first_str :f64 = f64::from_str(&first).unwrap();

    let last: &String = &vec_prices.last().unwrap().to_string();
    let last_str :f64 = f64::from_str(&last).unwrap();

    println!("variation: {}", (last_str - first_str) * 100.0 / first_str);

    return (last_str - first_str) * 100.0 / first_str;
}


