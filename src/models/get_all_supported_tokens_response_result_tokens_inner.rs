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
pub struct GetAllSupportedTokensResponseResultTokensInner {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "decimals")]
    pub decimals: i32,
    #[serde(rename = "supported")]
    pub supported: bool,
    #[serde(rename = "metadata", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl GetAllSupportedTokensResponseResultTokensInner {
    pub fn new(address: String, chain_id: i32, name: String, symbol: String, decimals: i32, supported: bool) -> GetAllSupportedTokensResponseResultTokensInner {
        GetAllSupportedTokensResponseResultTokensInner {
            address,
            chain_id,
            name,
            symbol,
            decimals,
            supported,
            metadata: None,
        }
    }
}
