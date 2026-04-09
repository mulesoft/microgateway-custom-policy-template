// Copyright 2026 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::{anyhow, Result};

use pdk::hl::*;
use pdk::logger;

use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, config: &Config) {
    let headers_state = request_state.into_headers_state().await;
    let token = headers_state.handler().header("Token").unwrap_or_default();
    // Log the header value
    logger::info!("Header value: {token}");

    // Add a new header
    headers_state
        .handler()
        .set_header("x-added", &config.string_property);
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config: Config = serde_json::from_slice(&bytes).map_err(|err| {
        anyhow!(
            "Failed to parse configuration '{}'. Cause: {}",
            String::from_utf8_lossy(&bytes),
            err
        )
    })?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}

#[cfg(test)]
mod test {
    use pdk_unit::{UnitTestBuilder, TraceBackend, UnitHttpMessage, UnitHttpRequest, UnitHttpResponse};
    use serde_json::json;
    use std::rc::Rc;

    // A custom backend that returns a 202 response.
    fn custom_backend(_req: UnitHttpRequest) -> UnitHttpResponse {
        UnitHttpResponse::new(202)
    }

    #[test]
    fn test_request_filter() {
        // Create a mock backend that will record incoming requests and produce custom responses.
        let backend = Rc::new(TraceBackend::new(custom_backend));

        // Create a tester with the custom backend, the policy to test, and its configuration.
        let mut tester = UnitTestBuilder::default()
            .with_config(json!({"stringProperty": "custom"}).to_string())
            .with_backend(Rc::clone(&backend))
            .with_entrypoint(super::configure);

        // Send a GET request to the policy and verify the response.
        let response = tester.request(UnitHttpRequest::get());
        assert_eq!(response.status_code(), 202);

        // We obtain the request that reached the backend and verify the header was added.
        let request = backend.next().unwrap();
        assert_eq!(request.header("x-added"), Some("custom"));
    }
}
