# ROFLPROXY

> When you are laughing on the floor, by proxy.

NOTHING TO SEE HERE!

This is a way for me to play with Rust and async-std, async-rs etc.

## Main goals of this project

- Play.
- Have fun.
- A tiny API gateway or proxy that works with static file API definition.
- Has SSO feature and works with OpenID Connect protocol, esp.**distributed claims** setup.
- Support multiple Identity Providers.
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
- Edit all `?` with proper crap
- 