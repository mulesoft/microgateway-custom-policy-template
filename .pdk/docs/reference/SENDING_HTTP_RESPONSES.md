# Reference for policy development

## Sending HTTP responses
Whenever you want to send an immediate response, use the `send_response` method to send back an HTTP response to the downstream,
specifying the status code, a list of response headers and, optionally, a response payload.

In case no payload is needed in the response, set the None value to it.

This action can only be performed when a request is received.
```rust
use pdk::api::classy::event::{EventData, Exchange, HeadersAccessor, RequestHeaders};
use pdk::api::logger;

const API_KEY_HEADER: &str = "api-key";

#[pdk::api::entrypoint]  
async fn filter(exchange: Exchange<RequestHeaders>) {    
    // Let the request pass through only if the API_KEY_HEADER is present 
    if exchange  
        .event_data()  
        .and_then(|event| event.header(API_KEY_HEADER))  
        .is_none() {
            // Access denied
            exchange.send_response(  
                401,  
                vec![("www-authenticate", "Basic")],  
                Some(&Vec::<u8>::from("{\"error\": \"missing api-key header\"}"))  
            );  
        }
        // Access granted
}
```
