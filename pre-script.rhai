// Copyright 2023 Salesforce, Inc. All rights reserved.

const PDK_SOURCE = "pdk_source";
const PDK_TEST_SOURCE = "pdk_test_source";
const PDK_VERSION = "pdk_version";

if variable::is_set(PDK_SOURCE) {
} else if variable::is_set(PDK_VERSION) {
    let version = variable::get(PDK_VERSION);
} else {
}

if !variable::is_set(PDK_TEST_SOURCE) {
}
