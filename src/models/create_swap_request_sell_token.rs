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
pub struct CreateSwapRequestSellToken {
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "address")]
    pub address: String,
}

impl CreateSwapRequestSellToken {
    pub fn new(chain_id: i32, address: String) -> CreateSwapRequestSellToken {
        CreateSwapRequestSellToken {
            chain_id,
            address,
        }
    }
}

