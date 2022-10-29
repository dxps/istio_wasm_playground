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
