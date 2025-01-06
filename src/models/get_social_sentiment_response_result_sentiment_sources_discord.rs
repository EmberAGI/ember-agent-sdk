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
pub struct GetSocialSentimentResponseResultSentimentSourcesDiscord {
    #[serde(rename = "mentions")]
    pub mentions: i32,
    #[serde(rename = "sentiment")]
    pub sentiment: f32,
}

impl GetSocialSentimentResponseResultSentimentSourcesDiscord {
    pub fn new(mentions: i32, sentiment: f32) -> GetSocialSentimentResponseResultSentimentSourcesDiscord {
        GetSocialSentimentResponseResultSentimentSourcesDiscord {
            mentions,
            sentiment,
        }
    }
}
