## Experiment 3

This experiment showcases how a WASM based extension of Istio's Envoy proxy can be made and be used.

From the usage perspective, here are the options:
- Access `echoserver`'s homepage using 
- Access the `/hello` path, which doesn't even reach `echoserver` since the filter returns its own response.<br/>
  - Use `curl -v http://localhost:9080/hello` to see that `x-filter` header is added in the response by the filter
  - Use `curl -v http://localhost:9080/hello -H "authority: snap"` to see additionally that the value is reflected in the `x-hello` response header.

<br/>

### Prerequisites

1. Have a local k3d cluster available.
2. Have Istio installed using the its `default` profile<br/>
   (tested with ver 1.15.1, available at the time of this writing)
3. Have `echoserver` installed, which means:
   1. Deployed: `k apply -f echoserver_deploy.yaml`
   2. Exposed in the cluster: `k apply -f echoserver_svc.yaml`
   3. Exposed the service to the outside world: `k apply -f echoserver_ingress.yaml`

<br/>

### Building

Briefly the steps:
- `cargo build --target wasm32-unknown-unknown --release`
- `cp target/wasm32-unknown-unknown/release/xp3_istio_wasme_rust_hello_header.wasm .`
- `wasme build precompiled xp3_istio_wasme_rust_hello_header.wasm --tag webassemblyhub.io/dxps/xp3_istio_wasme_rust_hello_header:v0.1`
- `wasme list`
- `wasme push webassemblyhub.io/dxps/xp3_istio_wasme_rust_hello_header:v0.1`

Notes:
- `runtime-config.json` file is being fetched from a generated asset using `wasme init . --language=rust --platform istio --platform-version 1.9.x`.

<br/>

### Deployment

Steps:
- `k delete envoyfilter,wasmplugin -n default --all`
- `k apply -f echoserver_wasmplugin.yaml`

Note that the version (tag) in `echoserver_wasmplugin.yaml` needs to be kept in sync with what's being used in the building phase above.

<br/>
