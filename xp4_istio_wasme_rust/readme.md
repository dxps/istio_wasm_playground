## Experiment 4

### `proxy-wasm`'s Concepts and Features

Proxy-Wasm supports two types of extensions:
1. Filters
   - An instance exists and is run within each (Envoy) worker thread.
2. Singleton services
   - One instance of it exists and gets executed in the main thread, outside the request lifecycle.

#### Contexts

For each request, a context object gets created. There are three types of contexts:
1. `RootContext`
   - This context can read the (WASM)VM configuration, read plugin configuration, set tick periods, create context, and any other work outside the request lifecycle.
   - One instance gets created for each thread in a plugin.
2. `StreamContext`
   - This context is used when writing network filters.
   - One instance gets created for each request.
3. `HTTPContext`
   - This context is used when writing HTTP filters.
   - One instance gets created for each request.1

#### Shared Data

Shared data is an in-memory key-value store, specified by the Proxy-Wasm ABI and provided by the proxy. Each VM contains a shared datastore.

The data exchanged between the host and VM is done in binary format, therefore, it requires serialization and deserialization.

<br/>

---

### Prereqs

These are the same as in experiment no. 3 (in `xp3_` sibling location).

<br/>

### Build & Publish

To build and publish, use `make tag=v0.1.4 all` as an example (publishing includes dxps account specific path).

Note that the `tag`'s value used for building and publishing must be reflected in `echoserver_wasmplugin.yaml` file before being deployed. Sure, this can be automated through scripting, to be considered.

<br/>

### Deploy

Use `k replace -f echoserver_wasmplugin.yaml` to "install" this plugin.<br/>
The alternative - that explicit the removal and then creation - is:
- `k delete -f echoserver_wasmplugin.yaml`
- `k apply -f echoserver_wasmplugin.yaml`

<br/>

### Usage / Test

From the usage perspective, here are the options:
- Use `curl -v http://localhost:9080` and you'll see at least the `x-sk-processor` header in the response, as provided by this plugin.
- Use `curl -v http://localhost:9080 -H "x-sk: abc` and you'll see at least the `x-sk` and `x-sk-processor` headers in the response, as provided by this plugin.
  - The value of `x-sk` response header contains the current and previous value, if any.<br/>
    Example:
    ```shell
    ❯ curl -v http://localhost:9080 -H "x-sk: abc" 2>&1 | grep "< x-sk"
    < x-sk: "abc"|"123"
    < x-sk-processor: xp4_istio_wasme_rust #3
    ❯
    ```
- Use `curl -v http://localhost:9080/hello` (optionally with that `x-sk` header in the request, as in the previous example) and you'll see that, besides these two headers, the response is provided by this plugin, not even reaching the target service.

<br/>

#### Logging

Note that by default the Envoy that runs into the `istio-proxy` container has the `info` logging level. If you want to see those debug level statements that are logged by the plugin, you need to update the logging level.<br/>
For this, use `istioctl pc log echoserver-v1-fcd7dc747-2ssm8 --level wasm:debug`<br/>
That `echoserver-v1-fcd7dc747-2ssm8` is just one of the pods where the sample app runs.

<br/>
