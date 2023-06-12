# Getting Started with Mulesoft's PDK
## Overview
The Policy Development Kit provides a simple mechanism to develop Rust custom policies for Mulesoft's Flex.
It abstracts developers from the underlying asynchronous nature of L7 proxies by providing a simple async/await API while allowing them to focus on the business needs instead of dealing with proxies architecture concerns.

## Requesting a distribution form the xapi
```
curl "https://anypoint.mulesoft.com/servicemesh/xapi/v1/flex/sdk?version=beta-0.0.0" <AUTHORIZATION> -L > pdk-beta-0.1.0.zip
```
Using as `<AUTHORIZATION>` one of the below options depending on how your org is configured.
#### Authorization for the endpoint without MFA
- `-u 'user:password'`  

#### Authorization for the endpoint with MFA
1. Create a [connected app](https://anypoint.mulesoft.com/accounts/connectedApps) that `acts on its own behalf (client credentials)`.
2. No scopes necessary
3. Copy the client id and secret
4. Obtain the bearer token by executing the request
```
curl https://anypoint.mulesoft.com/accounts/api/v2/oauth2/token \
--data-urlencode 'grant_type=client_credentials' \
--data-urlencode 'client_id=<client-id>' \
--data-urlencode 'client_secret=<client-secret>'
```
5. Your authorization header will be
   `-H "Authorization: Bearer <token from previous step>"`


## Getting started with Mulesoft's PDK
We will follow the steps to set up a working environment for designing a custom policy with Mulesoft's brand new PDK.
This policy will show how to log a specific request header for every request received.

0 - Make sure you have all dependencies installed.
- [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)
- [Docker](https://docs.docker.com/engine/install/)
- [Rust](https://www.rust-lang.org/tools/install) (version 1.65.0 or higher)
- **wasm32-wasi**: `rustup target add wasm32-wasi`
- **cargo generate**: `cargo install cargo-generate`

1 - Download the beta zip distributable using the steps in the previous section.

2 - Unzip.
```bash
unzip pdk-beta-0.1.0.zip
```

3 - Generate a new project for your policy. The command line will prompt a message asking if the new policy requires external configuration, with the `true` value meaning it does, and `false` if it does not. Default answer is `true`.

This command will also initiate a git repository for your policy in ```<policy-name>``` folder.

```bash
cargo generate pdk-template -n <policy-name>
```

In ```<policy-name>/src``` folder you will find a ```lib.rs``` file where you can implement your policy.

4 - Go into the project folder and compile the policy.
```bash
cd <policy-name>
make build
```
If everything goes well, you’ll see something like this appear:
<span style="padding:10px; background-color:black;"><span style="color:green;">**Finished**</span>. release [optimized] target(s) in 20.80s</span>

Find the raw WASM binary in: `target/wasm32-wasi/release/<policy_name>.wasm`

5 - That's it. You now have a working policy that you can extend with custom business logic. 

In order to achieve this, inside the `<policy-name>/README.md` file you will find: 
- Instructions on how to deploy this policy to a local Flex instance to see it in action.
- A link to the complete documentation detailing available features, example ready-to-go-policies and more.
