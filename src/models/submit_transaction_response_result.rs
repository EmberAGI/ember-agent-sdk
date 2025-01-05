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
pub struct SubmitTransactionResponseResult {
    #[serde(rename = "transactionHash", skip_serializing_if = "Option::is_none")]
    pub transaction_hash: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

impl SubmitTransactionResponseResult {
    pub fn new() -> SubmitTransactionResponseResult {
        SubmitTransactionResponseResult {
            transaction_hash: None,
            status: None,
            timestamp: None,
        }
    }
}

