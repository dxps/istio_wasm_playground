## Experiment 2

This one had one main issue: `proxy-wasm` dependency on the Bazel based setup that `wasme` is using is some 0.1.x, older than 0.2 version, as declared in `Cargo.toml`. And having that, `wasm build ...` failed like this, obviously:

```
error[E0050]: method `on_http_response_headers` has 3 parameters but the declaration in trait `proxy_wasm::traits::HttpContext::on_http_response_headers` has 2
  --> filter.rs:63:33
   |
63 |     fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
   |                                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 parameters, found 3
   |
   = note: `on_http_response_headers` from trait: `fn(&mut Self, usize) -> proxy_wasm::types::Action`
```

Therefore, until solving that issue (properly refer to the `proxy-wasm` ver. 0.2.x) and update back the signature of those handlers to include that 3rd boolean argument.

And now it's another issue, still having the same cause as before, this time `wasm build ...` fails with:
```
[0 / 12] [Prepa] Creating source manifest for //:filter
INFO: From Compiling Rust cdylib filter (1 files):
error[E0407]: method `get_type` is not a member of trait `RootContext`
  --> filter.rs:37:5
   |
37 | /     fn get_type(&self) -> Option<ContextType> {
38 | |         Some(ContextType::HttpContext)
39 | |     }
   | |_____^ not a member of trait `RootContext`

error[E0433]: failed to resolve: use of undeclared type or module `ContextType`
  --> filter.rs:38:14
   |
38 |         Some(ContextType::HttpContext)
   |              ^^^^^^^^^^^ use of undeclared type or module `ContextType`

error[E0407]: method `create_http_context` is not a member of trait `RootContext`
  --> filter.rs:41:5
   |
41 | /     fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
42 | |         Some(Box::new(HttpHeaders { context_id }))
43 | |     }
   | |_____^ not a member of trait `RootContext`

error[E0412]: cannot find type `ContextType` in this scope
  --> filter.rs:37:34
   |
36 | impl RootContext for HttpHeadersRoot {
   |     - help: you might be missing a type parameter: `<ContextType>`
37 |     fn get_type(&self) -> Option<ContextType> {
   |                                  ^^^^^^^^^^^ not found in this scope

error: aborting due to 4 previous errors

```

