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
pub struct WithdrawalDetailsCollectedFees {
    #[serde(rename = "token0")]
    pub token0: String,
    #[serde(rename = "token1")]
    pub token1: String,
}

impl WithdrawalDetailsCollectedFees {
    pub fn new(token0: String, token1: String) -> WithdrawalDetailsCollectedFees {
        WithdrawalDetailsCollectedFees {
            token0,
            token1,
        }
    }
}
