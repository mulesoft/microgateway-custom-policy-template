// Copyright 2023 Salesforce, Inc. All rights reserved.
use pdk::api::expression::Expression;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Header {
    #[serde(rename(deserialize = "key"))]
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(alias = "inboundHeaders")]
    pub inbound_headers: Vec<Header>,

    #[serde(alias = "outboundHeaders")]
    pub outbound_headers: Vec<Header>,
}
