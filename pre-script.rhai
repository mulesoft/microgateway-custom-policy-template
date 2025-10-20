// Copyright 2023 Salesforce, Inc. All rights reserved.

const PDK_SOURCE = "pdk_source";
const PDK_TEST_SOURCE = "pdk_test_source";
const PDK_VERSION = "pdk_version";

if !variable::is_set(PDK_SOURCE) {
    if variable::is_set(PDK_VERSION) {
        let version = variable::get(PDK_VERSION);
        variable::set(PDK_SOURCE, `{ version = "${version}", registry = "anypoint" }`)
    } else {
        variable::set(PDK_SOURCE, `{ version = "1.6.0-rc.0", registry = "anypoint" }`)
    }
}

if !variable::is_set(PDK_TEST_SOURCE) {
    variable::set(PDK_TEST_SOURCE, variable::get(PDK_SOURCE))
}