# Reference for policy development

## Policy Configuration
The applied configuration is injected into the policy in the form of a byte array. 
In order to access it, the `configure` function must be declared with a parameter of type ``Configuration(bytes)``.

Use the library of your preference to parse the bytes, but we encourage you to use `serde_json` library, as shown below.
In case no configuration is received, this array of bytes will be empty.
If your policy doesn't need configuration you can just remove the configuration parameter from the `configure` function.

In case the configuration is missing or invalid, the function allows you to return an error indicating this and the deployment will not complete.

After the configuration is parsed into a proper struct, share a reference to the filter function so it to be used during request/response processing.
 
 ```rust
use pdk::api::classy::event::{Exchange, RequestHeaders};
use pdk::api::classy::bootstrap::Launcher;
use pdk::api::classy::Configuration;
use pdk::api::logger;

#[derive(Debug)]  
struct BasicAuthentication {
    #[serde(alias = "username")]
    user: String,
  
    #[serde(alias = "password")]
    password: String, 
}

#[pdk::api::entrypoint]
pub async fn configure(
    launcher: Launcher,
    Configuration(bytes): Configuration,
) -> anyhow::Result<()> {
    // Parsing configuration into the BasicAuthentication struct.
    // We are bubbling up the error so, if the parsing fails, 
    // the policy deployment will also do so.
    let config: BasicAuthentication = serde_json::from_slice(&bytes)?; 

	logger::info!("Successfully parsed policy configuration");
  
    // Sharing configuration to be used by the filter function
    launcher
        .launch(|exchange| filter(exchange, &config))
        .await;  
  
    // Since no error was thrown, the deployment was a success
    Ok(())
}

async fn filter(exchange: Exchange<RequestHeaders>, config: &BasicAuthentication)  { 
	// Process HTTP requests and responses based on given config
}
```

Each of the fields present in the configuration struct must also exist in the `manifest.yaml` file with their corresponding type. 
The field name in the `manifest.yaml` must coincide with the `alias` entered in the `serde` annotation.
For example, in this case, the `properties` section of the `manifest.yaml` would be:

```yaml
properties:
    username:
      type: string
    password:
      type: string
```

**Notes**: 
- If the `configure` function is used, the `filter` function must **not** be annotated with `#[pdk::api::entrypoint]`.
- You can also choose to define this function as void. This is useful if it is not possible to fail in this stage.   
  `async fn configure(launcher: Launcher, Configuration(bytes): Configuration) { ... }`