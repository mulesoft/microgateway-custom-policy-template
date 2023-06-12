// Copyright 2023 Salesforce, Inc. All rights reserved.
mod config;
use config::{Config, Header};

use anyhow::Result;

use pdk::api::expression::{Expression, Value};
use pdk::api::{
    classy::{
        bootstrap::Launcher,
        event::{Exchange, HeadersAccessor, RequestHeaders},
        Configuration,
    },
    logger,
};

fn inject_headers(
    config_headers: &[Header],
    accessor: &dyn HeadersAccessor,
    resolve: impl Fn(&Expression) -> Option<Value>,
) {
    for config_header in config_headers {
        if let Some(resolved_value) = resolve(&config_header.value) {
            if let Some(value) = resolved_value.as_str() {
                let name = &config_header.name;
                logger::info!(r#"Applying config header: name = "{name}", value = "{value}""#);
                accessor.set_header(name, value);
            }
        }
    }
}

async fn filter(exchange: Exchange<RequestHeaders>, config: &Config) {
    if let Some(event) = exchange.event_data() {
        // request headers event was read
        logger::info!("Applying header-injection-lite filter for request");

        let resolve_on_request = |e: &Expression| e.resolve_on_request_headers(&event).ok();
        inject_headers(&config.inbound_headers, &event, resolve_on_request);
    }

    // wait for response headers
    let exchange = exchange.wait_for_response_headers().await;

    if let Some(event) = exchange.event_data() {
        // response headers event was read
        logger::info!("Applying header-injection-lite filter for response");

        let resolve_on_response = |e: &Expression| e.resolve_on_response_headers(&event).ok();
        inject_headers(&config.outbound_headers, &event, resolve_on_response);
    }
}

#[pdk::api::entrypoint]
async fn configure(launcher: Launcher, Configuration(config_bytes): Configuration) -> Result<()> {
    logger::info!("starting configuration for header-injection-lite");

    let config = serde_json::from_slice(&config_bytes)?;

    launcher.launch(|e| filter(e, &config)).await?;

    Ok(())
}
