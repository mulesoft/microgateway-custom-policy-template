// Copyright 2026 Salesforce, Inc. All rights reserved.

const PDK_SOURCE = "pdk_source";
const PDK_TEST_SOURCE = "pdk_test_source";
const PDK_VERSION = "pdk_version";

const PROJECT_MODE = "project-mode";
const DEFAULT_PROJECT_MODE = "unified";

const FIPS = "fips";

const FLEX_GATEWAY_VERSION = "flex_gateway_version";
const DEFAULT_FLEX_GATEWAY_VERSION = "1.13.0";

if !variable::is_set(PDK_SOURCE) {
    if variable::is_set(PDK_VERSION) {
        let version = variable::get(PDK_VERSION);
        variable::set(PDK_SOURCE, `{ version = "${version}" }`)
    } else {
        variable::set(PDK_SOURCE, `{ version = "1.10.0" }`)
    }
}

if !variable::is_set(PDK_TEST_SOURCE) {
    variable::set(PDK_TEST_SOURCE, variable::get(PDK_SOURCE))
}

if !variable::is_set(PROJECT_MODE) {
    variable::set(PROJECT_MODE, DEFAULT_PROJECT_MODE)
}

if !variable::is_set(FIPS) {
    variable::set(FIPS, false)
}

if !variable::is_set(FLEX_GATEWAY_VERSION) {
    variable::set(FLEX_GATEWAY_VERSION, DEFAULT_FLEX_GATEWAY_VERSION)
}
