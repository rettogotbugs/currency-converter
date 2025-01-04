use reqwest::Error;
use serde::Deserialize;
use std::io::{self, Write};

#[derive(Debug, Deserialize)]
struct ExchangeRate {
    rates: std::collections::HashMap<String, f64>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Welcome to the Currency Converter!");

    let mut base_currency = String::new();
    print!("Enter base currency (e.g., USD): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut base_currency).unwrap();
    let base_currency = base_currency.trim();

    let mut target_currency = String::new();
    print!("Enter target currency (e.g., EUR): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut target_currency).unwrap();
    let target_currency = target_currency.trim();

    let api_url = format!(
        "https://api.exchangerate.host/latest?base={}&symbols={}",
        base_currency, target_currency
    );

    let response: ExchangeRate = reqwest::get(&api_url).await?.json().await?;
    if let Some(rate) = response.rates.get(target_currency) {
        println!(
            "1 {} = {:.2} {}",
            base_currency, rate, target_currency
        );
    } else {
        println!("Conversion failed. Please check the currency codes.");
    }

    Ok(())
}
