// Copyright 2023 Salesforce, Inc. All rights reserved.
use pdk::api::expression::Expression;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PolicyConfiguration {
    #[serde(alias = "tokenExtractor")]
    pub token_extractor: Expression,
    pub upstream: String,
    pub host: String,
    pub path: String,
    pub authorization: String,
}
