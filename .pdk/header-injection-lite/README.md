# Header injection lite
Example of a Flex's filter developed with Mulesoft's PDK showing how to inject dynamically generated headers to both requests and responses.

## Deploying the example
1. Compile and add policy files to your Flex runtime following the general documentation.
2. Add the policy binding to your API
```yaml
apiVersion: gateway.mulesoft.com/v1alpha1
kind: ApiInstance
metadata:
  name: ingress-http
spec:
  address: http://0.0.0.0:8081
  services:
    upstream:
      address: http://my.backend.endpoint:80
      routes:
        - config:
            destinationPath: /backend/path
  policies:
    - policyRef:
        name: header-injection-lite-example
      config:
        inboundHeaders:
          - key: inbound-header
            value: "#[uuid()]"
        outboundHeaders:
          - key: outbound-header
            value: outbound
```
3. Hit your endpoint 
```bash
curl http://127.0.0.1:8081 -v
```