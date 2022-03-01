use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Request {
    pub endpoint: String,
    pub body: serde_json::Value,
}
// ACCOUNTS--------------------------------------------------------------------------
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountCreateResponse {
    pub seed: String,
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundResponse {
    pub from: String,
    pub to: String,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountFundBody {
    pub seed: String,
    pub amount: u128,
    pub to: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceBody {
    pub account: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccountBalanceResponse {
    pub balance: u128,
}

// ASSETS------------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaData {
    pub userdata: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateClassBody {
    pub seed: String,
    pub metadata: MetaData,
    pub class_id: u16,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateClassResponse {
    pub class_id: u16,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateBody {
    pub seed: String,
    pub account: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub metadata: MetaData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenCreateResponse {
    pub class_id: u32,
    pub asset_id: u32,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenMintBody {
    pub seed: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenMintResponse {
    pub to: String,
    pub class_id: u32,
    pub asset_id: u32,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBalanceBody {
    pub account: String,
    pub class_id: u16,
    pub asset_id: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenBalanceResponse {
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenTransferBody {
    pub seed: String,
    pub from: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TokenTransferResponse {
    pub from: String,
    pub to: String,
    pub class_id: u16,
    pub asset_id: u16,
    pub amount: u128,
    pub who: String,
}

//CURRENCY---------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub class_id: u64,
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssueBody {
    pub seed: String,
    pub to: String,
    pub currency: Currency,
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssueResponse {
    pub currency: Currency,
    pub who: String,
    pub amount: u128,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssuanceBody {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyIssuanceResponse {
    pub amount: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyMintBody {
    pub amount: u128,
    pub currency: Currency,
    pub seed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyMintResponse {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyBurnBody {
    pub amount: u128,
    pub currency: Currency,
    pub seed: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencyBurnResponse {
    pub currency: Currency,
    pub amount: u128,
    pub who: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencySupplyBody {
    pub currency: Currency,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CurrencySupplyResponse {
    pub total_supply: u128,
}

//ESCROW-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowCreateBody {
    pub seed: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowCreateResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowDepositBody {
    pub seed: String,
    pub escrow: String,
    pub class_id: u16,
    pub asset_ids: Vec<u16>,
    pub amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowDepositResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowRefundBody {
    pub seed: String,
    pub escrow: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EscrowRefundResponse {
    pub escrow: String,
    pub operator: String,
    pub owner: String,
}

//MARKET--------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub enum AmountOp {
    Equal,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RateAction {
    Transfer,
    Mint,
    Burn,
    Has(AmountOp),
}

#[derive(Serialize, Deserialize, Clone)]
pub enum RateAccount {
    Market,
    Account(String),
    Buyer,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AssetRate {
    class_id: u64,
    asset_id: u64,
    action: RateAction,
    amount: i128,
    from: RateAccount,
    to: RateAccount,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RateBalance {
    rate: AssetRate,
    balance: i128,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AmountOpInput {
    Transfer,
    Mint,
    Burn,
    HasEqual,
    HasLessThan,
    HasLessEqualThan,
    HasGreaterThan,
    HasGreaterEqualThan,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct AssetRateInput {
    class_id: u64,
    asset_id: u64,
    action: AmountOpInput,
    amount: i128,
    from: String,
    to: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct RatesInput {
    rates: Vec<AssetRateInput>,
    metadata: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateMarketBody {
    seed: String,
    market_id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMarketResponse {
    market_id: u64,
    who: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMarketRateBody {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    rates: RatesInput,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMarketRateResponse {
    market_id: u64,
    market_rate_id: u64,
    who: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DepositAssetsBody {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DepositAssetsResponse {
    who: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
    balances: Vec<RateBalance>,
    success: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExchangeAssetsBody {
    seed: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExchangeAssetsResponse {
    buyer: String,
    market_id: u64,
    market_rate_id: u64,
    amount: u128,
    balances: Vec<RateBalance>,
    success: bool,
}

//BUNDLE----------------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct BundleSchema {
    class_ids: Vec<u64>,
    asset_ids: Vec<Vec<u64>>,
    amounts: Vec<Vec<u128>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterBundleBody {
    seed: String,
    class_id: u64,
    asset_id: u64,
    bundle_id: String,
    schema: BundleSchema,
    metadata: serde_json::Value,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RegisterBundleResponse {
    bundle_id: String,
    who: String,
    class_id: u64,
    asset_id: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MintBundleBody {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MintBundleResponse {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleBody {
    seed: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

#[derive(Serialize, Deserialize)]
pub struct BurnBundleResponse {
    who: String,
    from: String,
    to: String,
    bundle_id: String,
    amount: u128,
}

//DEX----------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct CreateDexInput {
    seed: String,
    exchange_id: u32,
    currency: Currency,
    asset_class_id: u64,
    lp_class_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDexOutput {
    exchange_id: u32,
    who: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    max_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct BuyAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_out: Vec<u128>,
    currency_amounts_in: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsInput {
    seed: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    min_currency: u128,
    to: String,
}

#[derive(Serialize, Deserialize)]
pub struct SellAssetsOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts_in: Vec<u128>,
    currency_amounts_out: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    max_currencies: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct AddLiquidityOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityInput {
    seed: String,
    to: String,
    exchange_id: u32,
    asset_ids: Vec<u64>,
    liquidities: Vec<u128>,
    min_currencies: Vec<u128>,
    min_assets: Vec<u128>,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveLiquidityOutput {
    exchange_id: u32,
    who: String,
    to: String,
    asset_ids: Vec<u64>,
    asset_amounts: Vec<u128>,
    currency_amounts: Vec<u128>,
}

//VALIDATOR-----------------------------------------------------------------------------

#[derive(Serialize, Deserialize)]
pub struct AddValidatorInput {
    seed: String,
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct AddValidatorOutput {
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorInput {
    seed: String,
    validator_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct RemoveValidatorOutput {
    validator_id: String,
}
