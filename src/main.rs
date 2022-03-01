use requests::*;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::prelude::*;
use structs::*;

mod requests;
mod structs;

async fn print_header(
    title: &str,
    body: serde_json::Value,
    path: String,
) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.trim())
        .unwrap();
    writeln!(file, "Request: {}", title)?;
    println!("Request: {}", title);
    writeln!(file, "Body: {}", body.to_string())?;
    println!("Body: {}", body.to_string());
    Ok(())
}

async fn send_request(request: &Request, path: String) -> Result<String, Box<dyn Error>> {
    let res;
    match request.endpoint.as_str() {
        "account/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_create(request.endpoint.as_str(), body).await?;
            print_header("Account Create", request.body.clone(), path).await?;
        }
        "account/fund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            // let body = AccountFundBody {
            //     seed: serde_json::from_value(request.body.get("seed").unwrap().clone()).unwrap(),
            //     amount: serde_json::from_value(request.body.get("amount").unwrap().clone())
            //         .unwrap(),
            //     to: serde_json::from_value(request.body.get("to").unwrap().clone()).unwrap(),
            // };
            res = account_fund(request.endpoint.as_str(), body).await?;
            print_header("Account Fund", request.body.clone(), path).await?;
        }
        "account/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = account_balance(request.endpoint.as_str(), body).await?;
            print_header("Account Balance", request.body.clone(), path).await?;
        }
        "asset/create_class" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create_class(request.endpoint.as_str(), body).await?;
            print_header("Asset Create Class", request.body.clone(), path).await?;
        }
        "asset/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_create(request.endpoint.as_str(), body).await?;
            print_header("Asset Create", request.body.clone(), path).await?;
        }
        "asset/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_mint(request.endpoint.as_str(), body).await?;
            print_header("Asset Mint", request.body.clone(), path).await?;
        }
        "asset/balance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_balance(request.endpoint.as_str(), body).await?;
            print_header("Asset Balance", request.body.clone(), path).await?;
        }
        "asset/transfer_from" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = asset_transfer(request.endpoint.as_str(), body).await?;
            print_header("Asset Transfer From", request.body.clone(), path).await?;
        }
        "currency/issue" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issue(request.endpoint.as_str(), body).await?;
            print_header("Currency Issue", request.body.clone(), path).await?;
        }
        "currency/issuance" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_issuance(request.endpoint.as_str(), body).await?;
            print_header("Currency Issuance", request.body.clone(), path).await?;
        }
        "currency/mint" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_mint(request.endpoint.as_str(), body).await?;
            print_header("Currency Mint", request.body.clone(), path).await?;
        }
        "currency/burn" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_burn(request.endpoint.as_str(), body).await?;
            print_header("Currency Burn", request.body.clone(), path).await?;
        }
        "currency/supply" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = currency_supply(request.endpoint.as_str(), body).await?;
            print_header("Currency Supply", request.body.clone(), path).await?;
        }
        "escrow/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_create(request.endpoint.as_str(), body).await?;
            print_header("Escrow Create", request.body.clone(), path).await?;
        }
        "escrow/deposit" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_deposit(request.endpoint.as_str(), body).await?;
            print_header("Escrow Deposit", request.body.clone(), path).await?;
        }
        "escrow/refund" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = escrow_refund(request.endpoint.as_str(), body).await?;
            print_header("Escrow Refund", request.body.clone(), path).await?;
        }
        "market/create_market" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = market_create_market(request.endpoint.as_str(), body).await?;
            print_header("Create Market", request.body.clone(), path).await?;
        }
        "market/create_market_rate" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = market_create_market_rate(request.endpoint.as_str(), body).await?;
            print_header("Create Market Rate", request.body.clone(), path).await?;
        }
        "market/deposit_assets" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = market_deposit_assets(request.endpoint.as_str(), body).await?;
            print_header("Deposit Assets", request.body.clone(), path).await?;
        }
        "market/exchange_assets" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = market_exchange_assets(request.endpoint.as_str(), body).await?;
            print_header("Exchange Assets", request.body.clone(), path).await?;
        }
        "bundle/register_bundle" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = bundle_register_bundle(request.endpoint.as_str(), body).await?;
            print_header("Register Bundle", request.body.clone(), path).await?;
        }
        "bundle/mint_bundle" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = bundle_mint_bundle(request.endpoint.as_str(), body).await?;
            print_header("Mint Bundle", request.body.clone(), path).await?;
        }
        "bundle/burn_bundle" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = bundle_burn_bundle(request.endpoint.as_str(), body).await?;
            print_header("Burn Bundle", request.body.clone(), path).await?;
        }
        "dex/create" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = dex_create(request.endpoint.as_str(), body).await?;
            print_header("Dex Create", request.body.clone(), path).await?;
        }
        "dex/add_liquidity" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = dex_add_liquidity(request.endpoint.as_str(), body).await?;
            print_header("Dex Add Liquidity", request.body.clone(), path).await?;
        }
        "dex/remove_liquidity" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = dex_remove_liquidity(request.endpoint.as_str(), body).await?;
            print_header("Dex Remove Liquidity", request.body.clone(), path).await?;
        }
        "dex/buy_assets" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = dex_buy_assets(request.endpoint.as_str(), body).await?;
            print_header("Dex Buy Assets", request.body.clone(), path).await?;
        }
        "dex/sell_assets" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = dex_sell_assets(request.endpoint.as_str(), body).await?;
            print_header("Dex Sell Assets", request.body.clone(), path).await?;
        }
        "validator/add_validator" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = validator_add_validator(request.endpoint.as_str(), body).await?;
            print_header("Add Validator", request.body.clone(), path).await?;
        }
        "validator/remove_validator" => {
            let body = serde_json::from_value(request.body.clone()).unwrap();
            res = validator_remove_validator(request.endpoint.as_str(), body).await?;
            print_header("Remove Validator", request.body.clone(), path).await?;
        }

        _ => res = "This endpoint doesn't exist".to_string(),
    }
    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut path = String::from("option");
    while path != "" {
        //Create the variable to store the file name
        println!("Enter the file path/name: ");
        let mut file_name = String::new();
        io::stdin()
            .read_line(&mut file_name)
            .expect("Failed to read file name");
        let _ = &file_name.pop();
        path = file_name;

        //Read the file.json and create a vector of requests
        let file = File::open((path.clone() + ".json").trim())?;
        let requests: Vec<Request> = serde_json::from_reader(file)?;
        let iterator = requests.iter();
        File::create(path.clone() + ".txt").unwrap();

        //Execute the request read from the file
        for request in iterator {
            let res = send_request(request, path.clone() + ".txt").await?;
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open((path.clone() + ".txt").trim())
                .unwrap();

            writeln!(file, "Response: {}\n", res)?;
            println!("Response: {}\n", res);
        }
    }
    Ok(())
}
