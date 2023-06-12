# Example policies using Mulesoft's PDK

## Authentication policy

The aim of this policy is to show how to use the HTTP Client for authenticating an incoming request against an authentication server.
It also shows how to parse and evaluate a given Dataweave expression for extracting an auth token from the request.

If the request is successfully validated, access to the underlying API is allowed, while a 401 Unauthorized is responded otherwise.

The policy will expect the configuration to contain the mentioned DW expression and the Oauth2 server parameters.

For each incoming request:
- A token is extracted via the configured Dataweave expression. If the token is missing or the expression is malformed, the request is rejected.
- A request to the configured Oauth2 server is made. If the request fails to connect or if it times out, the incoming request is also rejected.
- If the Oauth server response body indicates that the token is active and non expired, the incoming request is permitted to access the API, otherwise it is rejected.

The possible errors the might arise in the policy are shown in the `FilterError` enum.

See the source code here: [Simple Oauth2 validation example policy](./../../simple-oauth2-validation). 
We also encourage you to deploy and try the policy in a Flex instance!
