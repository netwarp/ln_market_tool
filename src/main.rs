mod core;

use std::time::Duration;
use crate::core::{
    get_conf_file,
    get_last_ticker,
    get_last_price_from_ticker,
    get_variation_from_prices,
};

#[tokio::main]
async fn main() {

    match get_conf_file().await {
        Ok(config) => {
            println!("Configuration:\n{:?}", config);

            let threshold_down = &config["threshold_down"].as_float().unwrap();
            let threshold_up = &config["threshold_up"].as_float().unwrap();
            let minute_interval = &config["minute_interval"].as_integer().unwrap();

            let mut prices = Vec::new();

            loop {
                match get_last_ticker().await {
                    Ok(json_data) => {
                        match get_last_price_from_ticker(&json_data) {
                            Some(last_price) => {
                                //println!("Last Price: {}", last_price);
                                prices.push(last_price);
                                println!("{prices:?}");

                                if prices.len() > 3 {
                                    prices.remove(0);
                                }

                                let variation = get_variation_from_prices(prices.clone());
                                if variation <= *threshold_down || variation >= *threshold_up {
                                    println!("⚠️");
                                } else {
                                    println!("nothing to do");
                                }
                            }
                            None => {
                                eprintln!("Error: Could not extract last price from JSON.");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }

                tokio::time::sleep(Duration::from_secs((60 * minute_interval) as u64)).await;
            }


        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}