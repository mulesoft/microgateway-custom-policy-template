# Example policies using Mulesoft's PDK

## Transformation policy

The aim of this policy is to show how to access and modify request and response's headers.
It also shows how to parse and evaluate a Dataweave expressions for obtaining the header values to be injected.

The policy will expect a configuration detailing the list of headers to inject for both request and responses (named inbound and outbound headers), each one with
an associated Dataweave expression that compute its corresponding header value. 

For each incoming request:
- All inbound headers along with its corresponding dataweave expression are iterated.
- The expressions are evaluated, obtaining the expected string values. Headers whose DW expressions resolve to non-string values are ignored and won't be added.
- All headers with their computed values are applied to the request headers.
- Apply the same logic for the response.

See the source code here: [Header Injection Lite](./../../header-injection-lite).
We also encourage you to deploy and try the policy in a Flex instance!
