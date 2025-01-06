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
pub struct GetSocialSentimentResponseResultSentimentSources {
    #[serde(rename = "twitter", skip_serializing_if = "Option::is_none")]
    pub twitter: Option<Box<models::GetSocialSentimentResponseResultSentimentSourcesTwitter>>,
    #[serde(rename = "reddit", skip_serializing_if = "Option::is_none")]
    pub reddit: Option<Box<models::GetSocialSentimentResponseResultSentimentSourcesReddit>>,
    #[serde(rename = "discord", skip_serializing_if = "Option::is_none")]
    pub discord: Option<Box<models::GetSocialSentimentResponseResultSentimentSourcesDiscord>>,
}

impl GetSocialSentimentResponseResultSentimentSources {
    pub fn new() -> GetSocialSentimentResponseResultSentimentSources {
        GetSocialSentimentResponseResultSentimentSources {
            twitter: None,
            reddit: None,
            discord: None,
        }
    }
}
