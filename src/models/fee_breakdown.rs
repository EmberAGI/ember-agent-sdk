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
pub struct FeeBreakdown {
    #[serde(rename = "gasFee")]
    pub gas_fee: String,
    #[serde(rename = "serviceFee")]
    pub service_fee: String,
    #[serde(rename = "slippageCost")]
    pub slippage_cost: String,
    #[serde(rename = "total")]
    pub total: String,
}

impl FeeBreakdown {
    pub fn new(gas_fee: String, service_fee: String, slippage_cost: String, total: String) -> FeeBreakdown {
        FeeBreakdown {
            gas_fee,
            service_fee,
            slippage_cost,
            total,
        }
    }
}

