// Copyright 2023 Salesforce, Inc. All rights reserved.

const PDK_SOURCE = "pdk_source";
const PDK_TEST_SOURCE = "pdk_test_source";
const PDK_VERSION = "pdk_version";

// Debug: Log the current state of variables
print("DEBUG: PDK_SOURCE is set: " + variable::is_set(PDK_SOURCE));
if variable::is_set(PDK_SOURCE) {
    print("DEBUG: PDK_SOURCE value: " + variable::get(PDK_SOURCE));
}

print("DEBUG: PDK_TEST_SOURCE is set: " + variable::is_set(PDK_TEST_SOURCE));
if variable::is_set(PDK_TEST_SOURCE) {
    print("DEBUG: PDK_TEST_SOURCE value: " + variable::get(PDK_TEST_SOURCE));
}

if !variable::is_set(PDK_SOURCE) {
    print("DEBUG: Setting PDK_SOURCE to default value");
    if variable::is_set(PDK_VERSION) {
        let version = variable::get(PDK_VERSION);
        variable::set(PDK_SOURCE, `{ version = "${version}" }`);
        print("DEBUG: Set PDK_SOURCE to version: " + version);
    } else {
        variable::set(PDK_SOURCE, `{ version = "1.6.0-rc.0" }`);
        print("DEBUG: Set PDK_SOURCE to default version: 1.6.0-rc.0");
    }
}

if !variable::is_set(PDK_TEST_SOURCE) {
    print("DEBUG: Setting PDK_TEST_SOURCE to PDK_SOURCE value");
    variable::set(PDK_TEST_SOURCE, variable::get(PDK_SOURCE));
}

// Final debug: Log final values
print("DEBUG: Final PDK_SOURCE: " + variable::get(PDK_SOURCE));
print("DEBUG: Final PDK_TEST_SOURCE: " + variable::get(PDK_TEST_SOURCE));
