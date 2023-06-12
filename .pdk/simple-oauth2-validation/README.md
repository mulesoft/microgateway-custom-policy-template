# Simple oauth2 validation policy
A basic policy example that extracts an oauth2 token from the authentication header, and validates it against a rfc7662 compliant introspection endpoint.

# Deploying the example
1. Compile and add policy files to your Flex runtime following the general documentation.
2. Add a service configuration file to enable the policy to communicate with it.
```yaml
---
apiVersion: gateway.mulesoft.com/v1alpha1
kind: Service
metadata:
    name: introspection
spec:
    address: https://my.introspection.endpoint:5001
```

**Note**: If you don't have a functional introspection endpoint you can mock it with netcat
```bash
echo -e 'HTTP/1.1 200 OK\n\n {"active":true}' | nc -l 5001
```

3. Add the policy binding to your API 
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
        name: simple-oauth2-validation
      config:
        tokenExtractor: "#[dw::core::Strings::substringAfter(attributes.headers['Authorization'], 'Bearer ')]"
        upstream: introspection.default.svc
        host: my.introspection.endpoint:5001
        path: /authorize
        authorization: Basic dXNlcjpwYXNz
```
4. Hit your endpoint
```bash
curl http://127.0.0.1:8081 -H "Authorization: Bearer <your.oauth2.token>" -v
```