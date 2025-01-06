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
pub struct WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken {
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "amount")]
    pub amount: String,
}

impl WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken {
    pub fn new(address: String, amount: String) -> WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken {
        WithdrawLentTokensResponseResultWithdrawalDetailsWithdrawnToken {
            address,
            amount,
        }
    }
}
