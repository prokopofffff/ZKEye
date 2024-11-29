use methods::{HELLO_GUEST_ELF, HELLO_GUEST_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::error::Error;


#[tokio::main]
async fn main() {
    let input: u32 = 15 * u32::pow(2, 27) + 1;

    let env = ExecutorEnv::builder().write(&input).unwrap().build().unwrap();
    // let resp = reqwest::blocking::get("https://api-testnet.bybit.com/v5/market/tickers?category=spot&symbol=BTCUSDT").text();
    println!("I WAS HERE 1");
    get_json().await;
    println!("I WAS HERE 2");

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    println!("HELLO THERE I'M COMMITING HERE AYOOOOOOO");
    
    let receipt = prover.prove(env, HELLO_GUEST_ELF).unwrap().receipt;

    // Extract journal of receipt
    let output: u32 = receipt.journal.decode().unwrap();

    // Print, notice, after committing to a journal, the private input became public
    println!("Hello, world! I generated a proof of guest execution! {} is a public output from journal ", output);
}


// #[tokio::main]
async fn get_json() -> Result<(), Box<dyn std::error::Error>> {
    // Выполняем GET-запрос
    let response = reqwest::get("https://api-testnet.bybit.com/v5/market/tickers?category=spot&symbol=BTCUSDT")
        .await?
        .text()
        .await?;

    // Отладочный вывод полученного тела ответа
    println!("Raw response: {response}");

    // Попробуем парсить JSON
    match serde_json::from_str::<serde_json::Value>(&response) {
        Ok(parsed) => {
            println!("Parsed JSON: {:#?}", parsed);
        }
        Err(err) => {
            println!("Error parsing JSON: {err}");
        }
    }

    Ok(())
}