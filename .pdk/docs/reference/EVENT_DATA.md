# Reference for policy development

To access to the information relevant to the incoming request and its response you'll need to use interface EventData, which can be
accessed through the  `exchange.event_data` method.

## Request and Response Metadata
Use the event API to read the path, method, schema and authority of the incoming request and the status code from the response.  

This information is associated with the event data, so you will need to access it through the `exchange.event_data` method.
```rust
use pdk::api::classy::event::{Exchange, RequestHeaders};
use pdk::api::logger;

#[pdk::api::entrypoint]
async fn filter(exchange: Exchange<RequestHeaders>) {
    if let Some(event) = exchange.event_data() {
        logger::info!("Request method: {}", event.method());
        logger::info!("Request method: {}", event.method());
        logger::info!("Request scheme: {}", event.scheme());
        logger::info!("Request authority: {}", event.authority());
        logger::info!("Request path: {}", event.path());
        let exchange = exchange.wait_for_response_headers().await;
        if let Some(event) = exchange.event_data() {
            logger::info!("Response status code: {}", event.status_code());
        }
    }
}
```


## Headers manipulation
Use the event API to read, edit and delete request and response headers.

Headers access must be done with the associated event data, so it is needed to first access it through the `exchange.event_data` method. 
 ```rust
use pdk::api::classy::event::{EventData, Exchange, HeadersAccessor, RequestHeaders};
use pdk::api::logger;

#[pdk::api::entrypoint]
async fn filter(exchange: Exchange<RequestHeaders>) {
  if let Some(event) = exchange.event_data() {
    // Read a request header
    let trace_id = event.header("X-Trace-id").unwrap_or(String::from("unknown"));
    logger::info!("Request received from {}", trace_id);
    
    // Override a request header
    event.set_header("X-Powered-by", "Mulesoft");
    
    // Remove a request header
    event.remove_header("X-Trace-id");
  }

  // Wait for responses to come back 
  let exchange = exchange.wait_for_response_headers().await;

  if let Some(event) = exchange.event_data() {
    // API for manipulation of response headers is equivalent
    
    event.set_header("X-Served-by", "Mulesoft");
  }
}
```