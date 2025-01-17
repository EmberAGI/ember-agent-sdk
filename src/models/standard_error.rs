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
pub struct StandardError {
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "error")]
    pub error: Box<models::StandardErrorError>,
}

impl StandardError {
    pub fn new(success: bool, error: models::StandardErrorError) -> StandardError {
        StandardError {
            success,
            error: Box::new(error),
        }
    }
}

