// Copyright 2023 Salesforce, Inc. All rights reserved.
use pdk::api::classy::client::{HttpClientRequestError, HttpClientResponseError};

pub enum FilterError {
    Unexpected,
    NoToken,
    InactiveToken,
    ExpiredToken,
    NotYetActive,
    ClientRequestError(HttpClientRequestError),
    ClientResponseError(HttpClientResponseError),
    NonParsableIntrospectionBody(serde_json::Error),
}
