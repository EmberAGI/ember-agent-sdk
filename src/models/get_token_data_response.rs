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
pub struct GetTokenDataResponse {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "result")]
    pub result: Box<models::GetTokenDataResponseResult>,
}

impl GetTokenDataResponse {
    pub fn new(success: bool, result: models::GetTokenDataResponseResult) -> GetTokenDataResponse {
        GetTokenDataResponse {
            success,
            result: Box::new(result),
        }
    }
}
