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
pub struct WithdrawLentTokensRequest {
    #[serde(rename = "fromAddress")]
    pub from_address: String,
    #[serde(rename = "protocol")]
    pub protocol: String,
    #[serde(rename = "token")]
    pub token: String,
    #[serde(rename = "amount")]
    pub amount: String,
    #[serde(rename = "maxFee")]
    pub max_fee: String,
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

impl WithdrawLentTokensRequest {
    pub fn new(from_address: String, protocol: String, token: String, amount: String, max_fee: String) -> WithdrawLentTokensRequest {
        WithdrawLentTokensRequest {
            from_address,
            protocol,
            token,
            amount,
            max_fee,
            recipient: None,
        }
    }
}
