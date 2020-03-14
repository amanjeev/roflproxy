# ROFLPROXY

When you are laughing on the floor.

## Main goals of this project

- A tiny fast API gateway that works with static file API definition.
- Support multiple Identity Providers.
- Has SSO feature and works with OpenID Connect protocol.
- Allows for middleware/plugins to enhance the request/response flow and extensibility.

## TODO

- Proxy HTTP spec changes to adhere to the standard
    - Headers
    - Response status codes
- Mapping of 
    - Config based target servers
    - Config based API setting
    - Better way to define servers in config and scheme
- OIDC SSO
- Hot configuration change so the gateway runs all the time
- Edit all `?` with proper catching of errors
- 