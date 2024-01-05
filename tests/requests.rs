// Copyright 2023 Salesforce, Inc. All rights reserved.

use httpmock::MockServer;
use pdk_test::{pdk_test, TestComposite};
use pdk_test::port::Port;
use pdk_test::services::flex::{FlexConfig, Flex};
use pdk_test::services::httpmock::{HttpMockConfig, HttpMock};

// Directory where the policies implementations are stored.
const POLICY_DIR: &str =  concat!(env!("CARGO_MANIFEST_DIR"), "/test/config/custom-policies");

// Directory with the common configurations for tests.
const COMMON_CONFIG_DIR: &str =  concat!(env!("CARGO_MANIFEST_DIR"), "/tests/common");

// Directory with the configurations for the `hello` test.
const HELLO_CONFIG_DIR: &str =  concat!(env!("CARGO_MANIFEST_DIR"), "/tests/requests/hello");

// Flex port for the internal test network
const FLEX_PORT: Port = 8081;

// This integration test shows how to build a test to compose a local-flex instance
// with a MockServer backend
#[pdk_test]
async fn hello() -> anyhow::Result<()> {

    // Configure a Flex service
    let flex_config = FlexConfig::builder()
        .version("1.5.1")
        .hostname("local-flex")
        .ports([FLEX_PORT])
        .config_mounts([
            (POLICY_DIR, "policy"),
            (COMMON_CONFIG_DIR, "common"),
            (HELLO_CONFIG_DIR, "hello"),
        ])
        .build();

    // Configure an HttpMock service
    let httpmock_config = HttpMockConfig::builder()
        .port(80)
        .version("latest")
        .hostname("backend")
        .build();

    // Compose the services
    let composite = TestComposite::builder()
        .with_service(flex_config)
        .with_service(httpmock_config)
        .build()
        .await?;

    // Get a handle to the Flex service
    let flex: Flex = composite.service()?;

    // Get an external URL to point the Flex service
    let flex_url = flex.external_url(FLEX_PORT).unwrap();

    // Get a handle to the HttpMock service
    let httpmock: HttpMock = composite.service()?;

    // Create a MockServer
    let mock_server = MockServer::connect_async(httpmock.socket()).await;

    // Mock a /hello request
    mock_server.mock_async(|when, then| {
        when.path_contains("/hello");
        then.status(202).body("World!");
    }).await;

    // Perform an actual request
    let response = reqwest::get(format!("{flex_url}/hello")).await?;

    // Assert on the response
    assert_eq!(response.status(), 202);

    Ok(())
}
