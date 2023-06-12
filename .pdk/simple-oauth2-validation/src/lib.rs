// Copyright 2023 Salesforce, Inc. All rights reserved.
use crate::config::PolicyConfiguration;
use crate::error::FilterError;
use crate::introspection::{IntrospectionResponse, IntrospectionResponseExtractor};
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::client::HttpClient;
use pdk::api::classy::event::{Exchange, RequestHeaders};
use pdk::api::classy::Configuration;
use pdk::api::logger::{debug, warn};
use std::time::{SystemTime, UNIX_EPOCH};

mod config;
mod error;
mod introspection;

async fn introspect_token(
    token: &str,
    config: &PolicyConfiguration,
    client: HttpClient,
) -> Result<IntrospectionResponse, FilterError> {
    let body =
        serde_urlencoded::to_string([("token", token)]).map_err(|_| FilterError::Unexpected)?;

    let headers = vec![
        ("content-type", "application/x-www-form-urlencoded"),
        ("Authorization", config.authorization.as_str()),
    ];

    client
        .request(config.upstream.as_str(), config.host.as_str())
        .path(config.path.as_str())
        .headers(headers)
        .body(body.as_bytes())
        .extractor(IntrospectionResponseExtractor)
        .post()
        .map_err(FilterError::ClientRequestError)?
        .await
        .map_err(FilterError::ClientResponseError)?
}

async fn do_filter(
    exchange: &Exchange<RequestHeaders>,
    config: &PolicyConfiguration,
    client: HttpClient,
) -> Result<(), FilterError> {
    let event_data = exchange.event_data().ok_or(FilterError::Unexpected)?;

    let result = config
        .token_extractor
        .resolve_on_request_headers(&event_data)
        .map_err(|_| FilterError::NoToken)?;
    let token = result.as_str().ok_or(FilterError::NoToken)?;

    let response = introspect_token(token, config, client).await?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| FilterError::Unexpected)?
        .as_secs();

    if !response.active {
        return Err(FilterError::InactiveToken);
    }

    if response.exp.map(|exp| now > exp).unwrap_or_default() {
        return Err(FilterError::ExpiredToken);
    }

    if response.nbf.map(|nbf| now < nbf).unwrap_or_default() {
        return Err(FilterError::NotYetActive);
    }

    Ok(())
}

fn unauthorized_response(exchange: Exchange<RequestHeaders>) {
    exchange.send_response(
        401,
        vec![("WWW-Authenticate", "Bearer realm=\"oauth2\"")],
        None,
    );
}

fn server_error_response(exchange: Exchange<RequestHeaders>) {
    exchange.send_response(500, vec![], None);
}

async fn filter(
    exchange: Exchange<RequestHeaders>,
    config: &PolicyConfiguration,
    client: HttpClient,
) {
    if let Err(err) = do_filter(&exchange, config, client).await {
        match err {
            FilterError::Unexpected => {
                warn!("Unexpected error occurred while processing the request.");
                server_error_response(exchange);
            }
            FilterError::NoToken => {
                debug!("No authorization token was provided.");
                unauthorized_response(exchange);
            }
            FilterError::InactiveToken => {
                debug!("Token is marked as inactive by the introspection endpoint.");
                unauthorized_response(exchange);
            }
            FilterError::ExpiredToken => {
                debug!("Expiration time on the token has been exceeded.");
                unauthorized_response(exchange);
            }
            FilterError::NotYetActive => {
                debug!(
                    "Token is not yet valid, since time set in the nbf claim has not been reached."
                );
                unauthorized_response(exchange);
            }
            FilterError::ClientRequestError(err) => {
                warn!(
                    "Error sending the request to the introspection endpoint. {:?}.",
                    err
                );
                server_error_response(exchange);
            }
            FilterError::ClientResponseError(err) => {
                warn!(
                    "Error processing the response from the introspection endpoint. {:?}.",
                    err
                );
                server_error_response(exchange);
            }
            FilterError::NonParsableIntrospectionBody(err) => {
                warn!(
                    "Error parsing the response from the introspection endpoint. {}.",
                    err
                );
                server_error_response(exchange);
            }
        }
    }
}

#[pdk::api::entrypoint]
async fn configure(launcher: Launcher, Configuration(bytes): Configuration) -> anyhow::Result<()> {
    let config = serde_json::from_slice(&bytes)?;

    launcher
        .launch(|exchange, client| filter(exchange, &config, client))
        .await?;
    Ok(())
}
