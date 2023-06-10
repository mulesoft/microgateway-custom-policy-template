// Copyright 2023 Salesforce, Inc. All rights reserved.
mod generated;

use anyhow::Result;
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::event::{Exchange, HeadersAccessor, RequestHeaders};
use pdk::api::classy::Configuration;
use pdk::api::logger;
use crate::generated::config::Config;

// This filter shows how to log a specific request header.
// You can extend the function and use the configurations exposed in config.rs file
async fn filter(exchange: Exchange<RequestHeaders>, _config: &Config) {
    if let Some(event) = exchange.event_data() {
        // Log the header value
        logger::info!("Header value: {}", event.header("Token").unwrap_or_default());
    }
}

#[pdk::api::entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> Result<()> {
    let config = serde_json::from_slice(&bytes)?;
    launcher.launch(|e| filter(e, &config)).await?;
    Ok(())
}
