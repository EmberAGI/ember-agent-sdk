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
pub struct MintTokensResponseResultMintedToken {
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "amount")]
    pub amount: String,
}

impl MintTokensResponseResultMintedToken {
    pub fn new(token_address: String, amount: String) -> MintTokensResponseResultMintedToken {
        MintTokensResponseResultMintedToken {
            token_address,
            amount,
        }
    }
}

