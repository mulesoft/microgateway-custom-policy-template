// Copyright 2023 Salesforce, Inc. All rights reserved.

const PDK_SOURCE = "pdk_source";
const PDK_VERSION = "pdk_version";

if variable::is_set(PDK_SOURCE) {
    // If pdk_source is set, use it directly
} else if variable::is_set(PDK_VERSION) {
    let version = variable::get(PDK_VERSION);
    variable::set(PDK_SOURCE, `{ version = "${version}", registry = "anypoint" }`)
} else {
    variable::set(PDK_SOURCE, `{ version = "1.6.0-rc.0", registry = "anypoint" }`)
}
