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
pub struct GetAllSupportedChainsResponseResultChainsInnerNativeCurrency {
    #[serde(rename = "symbol")]
    pub symbol: String,
    #[serde(rename = "decimals")]
    pub decimals: i32,
}

impl GetAllSupportedChainsResponseResultChainsInnerNativeCurrency {
    pub fn new(symbol: String, decimals: i32) -> GetAllSupportedChainsResponseResultChainsInnerNativeCurrency {
        GetAllSupportedChainsResponseResultChainsInnerNativeCurrency {
            symbol,
            decimals,
        }
    }
}

