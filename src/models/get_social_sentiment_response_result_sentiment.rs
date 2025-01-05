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
pub struct GetSocialSentimentResponseResultSentiment {
    #[serde(rename = "score")]
    pub score: f32,
    #[serde(rename = "volume")]
    pub volume: i32,
    #[serde(rename = "trending")]
    pub trending: bool,
    #[serde(rename = "sources")]
    pub sources: Box<models::GetSocialSentimentResponseResultSentimentSources>,
    #[serde(rename = "timeframe")]
    pub timeframe: String,
    #[serde(rename = "lastUpdated")]
    pub last_updated: String,
}

impl GetSocialSentimentResponseResultSentiment {
    pub fn new(score: f32, volume: i32, trending: bool, sources: models::GetSocialSentimentResponseResultSentimentSources, timeframe: String, last_updated: String) -> GetSocialSentimentResponseResultSentiment {
        GetSocialSentimentResponseResultSentiment {
            score,
            volume,
            trending,
            sources: Box::new(sources),
            timeframe,
            last_updated,
        }
    }
}

