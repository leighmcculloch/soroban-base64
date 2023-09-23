# Soroban Base64 URL Encoding

| :warning: Code in this repo is demo material only. It has not been audited. Do not use to hold, protect, or secure anything. |
|-----------------------------------------|

This repo contains [Soroban] contracts that demonstrate how to encode base64 url encode.

[Stellar]: https://stellar.org
[Soroban]: https://soroban.stellar.org

The contract in this repo accepts a fixed size bytes, presumed to be some 32-byte hash, and encodes it to a 43-byte base64 url encoded string.

## Costs

Encoding a 32-byte hash into base64 url form uses the following resources.

When using a statically allocated buffer of exact size:
- WASM Size = 1,066 bytes
- CPU Instructions = 1,048,023

This size and instruction cost is rather minimal, especially when compared to the same contract that doesn't do the base64 encode:
- WASM Size = 601 bytes
- CPU Instructions = 852,844

Meaning the base64 encode is only 465 bytes and 195,179 cpu instructions.

## Example

```
❯ soroban contract invoke --network local --source me --id CAOBK5RB74ZCYKJBUSWCKKXBYSXCL3ZEJBAZZ3NAS2SL45MI5BGG2JNH -- encode --hash 7b2274797065223a22776562617574686e2e676574222c2263687384a7831326
"eyJ0eXBlIjoid2ViYXV0aG4uZ2V0IiwiY2hzhKeDEyY"
```
