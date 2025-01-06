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
pub struct CreateSwapResponseResultEstimatedOutput {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    #[serde(rename = "token", skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateSwapResponseResultEstimatedOutput {
    pub fn new() -> CreateSwapResponseResultEstimatedOutput {
        CreateSwapResponseResultEstimatedOutput {
            amount: None,
            token: None,
            value: None,
        }
    }
}
