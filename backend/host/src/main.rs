use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

use std::error::Error;
use serde::Deserialize;

const BINANCE: u8 = 1;
const BINANCE_API: &str = "https://api.binance.com/api/v3/ticker/price?";

const BYBIT: u8 = 2;
const BYBIT_API: &str = "https://api-testnet.bybit.com/v5/market/tickers?category=spot&";


#[derive(serde::Deserialize)]
struct Api1Response {
    symbol: String,
    price: String,
}

#[derive(serde::Deserialize)]
struct Api2Response {
    result: Api2Result,
    time: u64,
}

#[derive(serde::Deserialize)]
struct Api2Result {
    list: Vec<Api2Ticker>,
}

#[derive(serde::Deserialize)]
struct Api2Ticker {
    symbol: String,
    usdIndexPrice: String,
    
}

#[tokio::main]
async fn main() {
    let input: u32 = 15 * u32::pow(2, 27) + 1;

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();

    println!("I WAS HERE 1");

    if let Ok((price_1, timestamp_1)) = api_reqwest(BINANCE_API, "BTCUSDT", BINANCE).await {
        println!("Binance BTCUSDT Price: {}, Timestamp: {}", price_1, timestamp_1);
    } else {
        println!("[!] Parsing of Binance failed");
    }

    // calling Bybit's API
    if let Ok((price_2, timestamp_2)) = api_reqwest(BYBIT_API, "BTCUSDT", BYBIT).await {
        println!("Bybit BTCUSDT Price: {}, Timestamp: {}", price_2, timestamp_2);
    } else {
        println!("[!] Parsing of Bybit failed");
    }

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.    
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap().receipt;

    // Extract journal of receipt
    let output: u32 = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Hello, world! I generated a proof of guest execution! {} is a public output from journal ", output);
}



async fn api_reqwest(
    api_url: &str,
    ticker: &str,
    api_format: u8,
) -> Result<(f64, u64), Box<dyn std::error::Error>> {
    let url = format!("{}symbol={}", api_url, ticker);

    let response = reqwest::get(&url).await?.text().await?;
    // println!("API Response: {}", response);

    match api_format {
        1 => {
            // @dev Binance parser
            match serde_json::from_str::<Api1Response>(&response) {
                Ok(parsed) => {
                    let timestamp = chrono::Utc::now().timestamp() as u64; // current timestamp 
                    println!("timestamp: {}", timestamp);
                    return Ok((parsed.price.parse::<f64>()?, timestamp));
                }
                Err(err) => {
                    println!("Error parsing BINANCE API response: {}", err);
                    return Err(Box::new(err));
                }
            }
        }
        2 => {
            // @dev Bybit parser
            match serde_json::from_str::<Api2Response>(&response) {
                Ok(parsed) => {
                    if let Some(ticker_data) = parsed.result.list.iter().find(|t| t.symbol == ticker)
                    {
                        return Ok((ticker_data.usdIndexPrice.clone().parse::<f64>()? , parsed.time / 1000));
                    }
                    Err("Ticker not found in BYBIT API response")?
                }
                Err(err) => {
                    println!("Error parsing BYBIT API  response: {}", err);
                    return Err(Box::new(err));
                }
            }
        }
        _ => Err("Unknown API format".into()),
    }
}
