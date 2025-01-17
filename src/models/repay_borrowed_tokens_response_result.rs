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
pub struct RepayBorrowedTokensResponseResult {
    /// Transaction details specific to the blockchain (e.g., userOp for ERC-4337, raw tx for EVM, instructions for Solana)
    #[serde(rename = "transactionPlan", skip_serializing_if = "Option::is_none")]
    pub transaction_plan: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "feeBreakdown", skip_serializing_if = "Option::is_none")]
    pub fee_breakdown: Option<Box<models::FeeBreakdown>>,
    #[serde(rename = "repaymentDetails", skip_serializing_if = "Option::is_none")]
    pub repayment_details: Option<Box<models::RepayBorrowedTokensResponseResultRepaymentDetails>>,
}

impl RepayBorrowedTokensResponseResult {
    pub fn new() -> RepayBorrowedTokensResponseResult {
        RepayBorrowedTokensResponseResult {
            transaction_plan: None,
            fee_breakdown: None,
            repayment_details: None,
        }
    }
}

