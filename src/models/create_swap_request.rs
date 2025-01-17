/*
 * Ember API
 *
 * Comprehensive specifications for all Ember API endpoints, incorporating the complete set of DeFi capabilities.
 *
 * The version of the OpenAPI document: 0.2
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateSwapRequest {
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "swapType")]
    pub swap_type: SwapType,
    #[serde(rename = "sellToken")]
    pub sell_token: Box<models::CreateSwapRequestSellToken>,
    #[serde(rename = "buyToken")]
    pub buy_token: Box<models::CreateSwapRequestSellToken>,
    #[serde(rename = "sellAmount")]
    pub sell_amount: String,
    #[serde(rename = "slippageTolerance")]
    pub slippage_tolerance: String,
    #[serde(rename = "maxFee")]
    pub max_fee: String,
}

impl CreateSwapRequest {
    pub fn new(from_address: String, swap_type: SwapType, sell_token: models::CreateSwapRequestSellToken, buy_token: models::CreateSwapRequestSellToken, sell_amount: String, slippage_tolerance: String, max_fee: String) -> CreateSwapRequest {
        CreateSwapRequest {
            from_address,
            swap_type,
            sell_token: Box::new(sell_token),
            buy_token: Box::new(buy_token),
            sell_amount,
            slippage_tolerance,
            max_fee,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SwapType {
    #[serde(rename = "MARKET_SELL")]
    MarketSell,
    #[serde(rename = "LIMIT_SELL")]
    LimitSell,
}

impl Default for SwapType {
    fn default() -> SwapType {
        Self::MarketSell
    }
}

