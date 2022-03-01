use crate::structs::*;
use std::error::Error;

const BASE_URL: &str = "http://127.0.0.1:4000/";

pub async fn account_create(endpoint: &str, body: String) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn account_fund(endpoint: &str, body: AccountFundBody) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn account_balance(
    endpoint: &str,
    body: AccountBalanceBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn asset_create_class(
    endpoint: &str,
    body: TokenCreateClassBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn asset_create(endpoint: &str, body: TokenCreateBody) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn asset_mint(endpoint: &str, body: TokenMintBody) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn asset_balance(
    endpoint: &str,
    body: TokenBalanceBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn asset_transfer(
    endpoint: &str,
    body: TokenTransferBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn currency_issue(
    endpoint: &str,
    body: CurrencyIssueBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn currency_issuance(
    endpoint: &str,
    body: CurrencyIssuanceBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn currency_mint(
    endpoint: &str,
    body: CurrencyMintBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn currency_burn(
    endpoint: &str,
    body: CurrencyBurnBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn currency_supply(
    endpoint: &str,
    body: CurrencySupplyBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn escrow_create(
    endpoint: &str,
    body: EscrowCreateBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn escrow_deposit(
    endpoint: &str,
    body: EscrowDepositBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn escrow_refund(
    endpoint: &str,
    body: EscrowRefundBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn market_create_market(
    endpoint: &str,
    body: CreateMarketBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn market_create_market_rate(
    endpoint: &str,
    body: CreateMarketRateBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn market_deposit_assets(
    endpoint: &str,
    body: DepositAssetsBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn market_exchange_assets(
    endpoint: &str,
    body: ExchangeAssetsBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn bundle_register_bundle(
    endpoint: &str,
    body: RegisterBundleBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn bundle_mint_bundle(
    endpoint: &str,
    body: MintBundleBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn bundle_burn_bundle(
    endpoint: &str,
    body: BurnBundleBody,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn dex_create(endpoint: &str, body: CreateDexInput) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn dex_buy_assets(
    endpoint: &str,
    body: BuyAssetsInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn dex_sell_assets(
    endpoint: &str,
    body: SellAssetsInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn dex_add_liquidity(
    endpoint: &str,
    body: AddLiquidityInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn dex_remove_liquidity(
    endpoint: &str,
    body: RemoveLiquidityInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn validator_remove_validator(
    endpoint: &str,
    body: RemoveValidatorInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}

pub async fn validator_add_validator(
    endpoint: &str,
    body: AddValidatorInput,
) -> Result<String, Box<dyn Error>> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let client = reqwest::Client::new();
    let res = client
        .post(&url)
        .body(serde_json::to_string(&body)?)
        .header("Content-Type", "application/json")
        .send()
        .await?
        .text()
        .await?;
    Ok(res)
}
