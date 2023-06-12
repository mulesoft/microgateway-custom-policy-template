// Copyright 2023 Salesforce, Inc. All rights reserved.
use crate::FilterError;
use pdk::api::classy::client::{HttpCallResponse, ResponseBuffers, ResponseExtractor};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IntrospectionResponse {
    pub active: bool,
    pub exp: Option<u64>,
    pub nbf: Option<u64>,
}

pub struct IntrospectionResponseExtractor;

impl ResponseExtractor for IntrospectionResponseExtractor {
    type Output = Result<IntrospectionResponse, FilterError>;

    fn extract(self, event: &HttpCallResponse, buffers: &dyn ResponseBuffers) -> Self::Output {
        if buffers.status_code() != 200 {
            Err(FilterError::InactiveToken)
        } else {
            let body = buffers
                .body(0, event.body_size)
                .ok_or(FilterError::Unexpected)?;
            serde_json::from_slice(body.as_slice())
                .map_err(FilterError::NonParsableIntrospectionBody)
        }
    }
}
