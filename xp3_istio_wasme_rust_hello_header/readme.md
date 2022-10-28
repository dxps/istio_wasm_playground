## Experiment 3

This experiment showcases how a WASM based extension of Istio's Envoy proxy can be made and be used.

<br/>

### Prerequisites

1. Have a local k3d cluster available.<br/>
   Without k3d provided (Traefik) gateway, since Istio's Ingress will be used:<br/>
   ```shell
   k3d cluster create iiac --servers 1 --agents 3                                    \
                           --port 9080:80@loadbalancer --port 9443:443@loadbalancer  \
                           --api-port 6443                                           \
                           --k3s-arg="--disable=traefik@server:0"
   ```
   <br/>

2. Have Istio installed using its `default` profile:<br/>
   `istioctl install --set profile=default`<br/>
   Tested with ver 1.15.1, available at the time of this writing.
   <br/><br/>

3. Have `echoserver` installed, which means:
   1. Deployed: `k apply -f echoserver_deploy.yaml`
   2. Exposed in the cluster: `k apply -f echoserver_svc.yaml`
   3. Exposed the service to the outside world: `k apply -f echoserver_ingress.yaml`
   4. And it would respond to an URL such as `http://localhost:9080`.

<br/>

### Building

Briefly, the steps are as follows:
- `cargo build --target wasm32-unknown-unknown --release`
- `cp target/wasm32-unknown-unknown/release/xp3_istio_wasme_rust_hello_header.wasm .`
- `wasme build precompiled xp3_istio_wasme_rust_hello_header.wasm --tag webassemblyhub.io/dxps/xp3_istio_wasme_rust_hello_header:v0.1`
- `wasme list`
- `wasme push webassemblyhub.io/dxps/xp3_istio_wasme_rust_hello_header:v0.1`

Notes:
- `runtime-config.json` file was being copied from the generated outcome of `wasme init . --language=rust --platform istio --platform-version 1.9.x`.

<br/>

### Deployment

Steps:
- `k delete wasmplugin -n default --all`
- `k apply -f echoserver_wasmplugin.yaml`

Note that the version (tag) in `echoserver_wasmplugin.yaml` file needs to be kept in sync with what's being built and pushed to Webassembly Hub.

<br/>

### Usage

From the usage perspective, here are the options:
- Use `curl -v http://localhost:9080` and you'll see `x-filtered-by` and `x-hello` headers in the response, as provided by this plugin.
- Use `curl -v http://localhost:9080/hello` and you'll see that, besides these two headers, the response is provided by this plugin, not even reaching the target service.
- Use `curl -v http://localhost:9080/hello -H "authority: whatever"` to see additionally that the value of that request header is reflected back in the `x-hello` response header.

<br/>

### Logging

Note that by default the Envoy that runs into the `istio-proxy` container has the `info` logging level. If you want to see those debug level statements that are logged by the plugin, you need to update the logging level.<br/>
For this, use `istioctl pc log echoserver-v1-fcd7dc747-2ssm8 --level wasm:debug`<br/>
That `echoserver-v1-fcd7dc747-2ssm8` is just one of the pods where the sample app runs.

<br/>
