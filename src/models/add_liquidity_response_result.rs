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
pub struct AddLiquidityResponseResult {
    /// Transaction details.
    #[serde(rename = "transactionPlan", skip_serializing_if = "Option::is_none")]
    pub transaction_plan: Option<serde_json::Value>,
    #[serde(rename = "feeBreakdown", skip_serializing_if = "Option::is_none")]
    pub fee_breakdown: Option<Box<models::FeeBreakdown>>,
    #[serde(rename = "positionDetails", skip_serializing_if = "Option::is_none")]
    pub position_details: Option<Box<models::AddLiquidityResponseResultPositionDetails>>,
}

impl AddLiquidityResponseResult {
    pub fn new() -> AddLiquidityResponseResult {
        AddLiquidityResponseResult {
            transaction_plan: None,
            fee_breakdown: None,
            position_details: None,
        }
    }
}
