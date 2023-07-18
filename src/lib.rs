// Copyright 2023 Salesforce, Inc. All rights reserved.
use anyhow::Result;

use pdk::hilux::*;

use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn request_filter(request_state: RequestState, _config: &Config) {
    let headers_state = request_state.into_headers_state().await;
    let token = headers_state.handler().header("Token").unwrap_or_default();
    // Log the header value
    logger::info!("Header value: {token}");
}

#[entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config = serde_json::from_slice(&bytes)?;
    let filter = on_request(|rs| request_filter(rs, &config));
    launcher.launch(filter).await?;
    Ok(())
}
