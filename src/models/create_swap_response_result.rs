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
pub struct CreateSwapResponseResult {
    /// Transaction details specific to the blockchain.
    #[serde(rename = "transactionPlan", skip_serializing_if = "Option::is_none")]
    pub transaction_plan: Option<serde_json::Value>,
    #[serde(rename = "feeBreakdown", skip_serializing_if = "Option::is_none")]
    pub fee_breakdown: Option<Box<models::FeeBreakdown>>,
    #[serde(rename = "estimatedOutput", skip_serializing_if = "Option::is_none")]
    pub estimated_output: Option<Box<models::CreateSwapResponseResultEstimatedOutput>>,
}

impl CreateSwapResponseResult {
    pub fn new() -> CreateSwapResponseResult {
        CreateSwapResponseResult {
            transaction_plan: None,
            fee_breakdown: None,
            estimated_output: None,
        }
    }
}
