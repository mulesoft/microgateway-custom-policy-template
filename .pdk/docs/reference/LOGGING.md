# Reference for policy development

## Logging

Use the `pdk::api::logger` for logging simple and complex information with several logging levels: `debug`, `info`, `warn` and `error`.

By using this logger, all logs will contain information about the associated API and Policy and a request id for better traceability.
 ```rust
 use std::fmt;
use std::fmt::Formatter;
use pdk::api::classy::event::{Exchange, RequestHeaders};
use pdk::api::logger;

#[derive(Debug)]
struct UserData<'a> {
    tracking_id: &'a str,
    country: &'a str,
    active: bool,
}

#[pdk::api::entrypoint]
async fn filter(exchange: Exchange<RequestHeaders>) {
    // Simplest log
    // [policy: <policy-name>][api: <api-name>][req: 2483af77-5b7e-4980-acee-9649f130c2b4] A request has been received
    logger::info!("A request has been received");

    let tracking_id = "6252393e-ca89-471d-8376-745c98a9e0fc"; // read tracking data from request

    // Parameterized log
    // [policy: <policy-name>][api: <api-name>][req: 2483af77-5b7e-4980-acee-9649f130c2b4] 6252393e-ca89-471d-8376-745c98a9e0fc has accessed the API
    logger::info!("{} has accessed the API", tracking_id);

    let user_data: UserData = UserData { tracking_id, country: "US", active: false }; // load user data for this tracking_id

    if !user_data.active {
        // Warning log
        // [policy: <policy-name>][api: <api-name>][req: 2483af77-5b7e-4980-acee-9649f130c2b4] Inactive user 6252393e-ca89-471d-8376-745c98a9e0fc is trying to access the API
        logger::warn!("Inactive user {} is trying to access the API", user_data.tracking_id);
    }

    // Complex structure log
    // [policy: <policy-name>][api: <api-name>][req: 2483af77-5b7e-4980-acee-9649f130c2b4] Associated user data for 6252393e-ca89-471d-8376-745c98a9e0fc: UserData { tracking_id: "6252393e-ca89-471d-8376-745c98a9e0fc", country: "US", active: false }
    logger::info!("Associated user data for {}: {:?}", tracking_id, user_data); // Using Debug format
    
    // [policy: <policy-name>][api: <api-name>][req: 2483af77-5b7e-4980-acee-9649f130c2b4] Associated user data for 6252393e-ca89-471d-8376-745c98a9e0fc: 6252393e-ca89-471d-8376-745c98a9e0fc from country US is inactive
    logger::info!("Associated user data for {}: {}", tracking_id, user_data); // Using Display format
}

impl fmt::Display for UserData<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} from country {} is {}", self.tracking_id, self.country, if self.active { "active" } else { "inactive" })
    }
}
```
