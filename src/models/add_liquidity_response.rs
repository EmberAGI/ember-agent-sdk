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
pub struct AddLiquidityResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "result")]
    pub result: Box<models::AddLiquidityResponseResult>,
}

impl AddLiquidityResponse {
    pub fn new(success: bool, result: models::AddLiquidityResponseResult) -> AddLiquidityResponse {
        AddLiquidityResponse {
            success,
            result: Box::new(result),
        }
    }
}
