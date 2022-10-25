use log::info;
use proxy_wasm as wasm;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(wasm::types::LogLevel::Trace);
    proxy_wasm::set_http_context(
        |context_id, _root_context_id| -> Box<dyn wasm::traits::HttpContext> {
            Box::new(IstioWasmRustHelloHeader { context_id })
        },
    )
}

struct IstioWasmRustHelloHeader {
    context_id: u32,
}

impl wasm::traits::Context for IstioWasmRustHelloHeader {}

impl wasm::traits::HttpContext for IstioWasmRustHelloHeader {
    fn on_http_request_headers(
        &mut self,
        num_headers: usize,
        _end_of_stream: bool,
    ) -> wasm::types::Action {
        info!("Got {} HTTP headers in #{}.", num_headers, self.context_id);
        let headers = self.get_http_request_headers();
        let mut authority = "";

        for (name, value) in &headers {
            if name == ":authority" {
                authority = value;
            }
        }

        self.set_http_request_header("x-hello", Some(&format!("Hello world from {}", authority)));

        wasm::types::Action::Continue
    }
}
