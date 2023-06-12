# Developing your policy

## Dataweave expressions evaluation

### Expression parsing
Policies that are configured with DataWeave expressions must use the `Expression` struct in order to evaluate the expression within the policy.

First of all, declare the associated configuration field in a configuration struct and parse the configuration into it.
The parse will fail if the expression is malformed.
```rust
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::Configuration;
use pdk::api::expression::Expression;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub expression: Expression,
}

#[pdk::api::entrypoint]
async fn configure(
    launcher: Launcher,
    Configuration(bytes): Configuration,
) -> anyhow::Result<()> {
    let config = serde_json::from_slice(bytes.as_slice())?;
    
    launcher
        .launch(|exchange| filter(&config, exchange)) //filter function defined below
        .await?;

    Ok(())
}
```

When using DataWeave in a policy configuration parameter, it must also be added to the policy `manifest.yaml` file.
In this case, the `properties` section of the `manifest.yaml` file would be:
```yaml
properties:
  expression:
      type: string
      format: dataweave
```

### Expression evaluation
Then, evaluate the expression within a given request/response with the associated event data.

The evaluation will return a `Result` since it can be successful or not, for example using an attribute that is missing in the request will make the evaluation to fail.
In case the evaluation succedes, the developer must explicit what datatype is expected in the result of the evaluation.

In the example below the `evaluation` is expected to extract a token from the request. Since the expected type is string, we use the `as_str` method, which will return
an Option indicating if the coercion was possible or not.
```rust
use pdk::api::classy::event::{Exchange, RequestHeaders};
use pdk::api::logger;

async fn filter(config: &Config, exchange: Exchange<RequestHeaders>) {
    if let Some(event) = exchange.event_data() {

        // Evaluate the expression for this given request
        match config.expression.resolve_on_request_headers(&event) {
            Ok(evaluation) => {

                // Try to use the result as a string
                match evaluation.as_str() {
                    None => logger::error!("The evaluation could not be turned into string!"),
                    Some(token) => {
                        logger::info!("The request contained to associated token {}", token);
                    }
                }
            }
            Err(err) => logger::warn!("Expression could not be resolved!")
        }
    }

    let exchange = exchange.wait_for_response_headers().await;

    if let Some(event) = exchange.event_data() {

        // Evaluate the expression for the associated response
        let evaluation = config.expression.resolve_on_response_headers(&event);

        // Process this evaluation the same way as before...
    }
}
```
### Intermediate representation language for expressions
While expressions are written at a high-level configuration point as DataWeave expressions, the `Expression` type is actually managing an intermediate representation that is generated after compiling DataWeave expressions during the policy deployment. When a configuration struct is being deserialized, a specialized deserializer parses the intermediate representation and instantiates the `Expression` type. 
