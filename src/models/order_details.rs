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
pub struct OrderDetails {
    #[serde(rename = "sellToken")]
    pub sell_token: Box<models::OrderDetailsSellToken>,
    #[serde(rename = "buyToken")]
    pub buy_token: Box<models::CreateLimitOrderRequestBuyToken>,
    #[serde(rename = "limitPrice")]
    pub limit_price: String,
}

impl OrderDetails {
    pub fn new(sell_token: models::OrderDetailsSellToken, buy_token: models::CreateLimitOrderRequestBuyToken, limit_price: String) -> OrderDetails {
        OrderDetails {
            sell_token: Box::new(sell_token),
            buy_token: Box::new(buy_token),
            limit_price,
        }
    }
}
