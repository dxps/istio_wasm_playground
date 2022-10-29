use std::string;

use log::debug;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> { Box::new(HttpHeadersRoot) });
}}

struct HttpHeadersRoot;

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders { context_id }))
    }
}

const SHARED_KEY_HEADER: &str = "sk";

struct HttpHeaders {
    context_id: u32,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _hdrs_cnt: usize, _eos: bool) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            debug!("#{} -> {}: {}", self.context_id, name, value);
        }

        if let Some(sk) = self.get_http_request_header(SHARED_KEY_HEADER) {
            let (data_opt, cas) = &self.get_shared_data(SHARED_KEY_HEADER);
            // cas (abreviation for compare-and-switch) is used when updating the value.
            debug!(
                "#{} -> received {}={:?}",
                self.context_id, SHARED_KEY_HEADER, data_opt
            );
            let resp_sk = match data_opt {
                Some(prev_data) => format!("{:?}|{:?}", prev_data, sk),
                None => sk,
            };
            match &self.set_shared_data(SHARED_KEY_HEADER, Some(resp_sk.as_bytes()), *cas) {
                Ok(_) => debug!(
                    "#{} -> set {}={}",
                    self.context_id, SHARED_KEY_HEADER, resp_sk
                ),
                Err(e) => debug!(
                    "#{} -> failed to set {} due to '{:?}'",
                    self.context_id, SHARED_KEY_HEADER, e
                ),
            }
        }

        match self.get_http_request_header(":path") {
            Some(path) if path == "/hello" => {
                self.send_http_response(
                    200,
                    vec![("Hello", "World"), ("Powered-By", "proxy-wasm")],
                    Some(b"Hello, World!\n"),
                );
                Action::Pause
            }
            _ => Action::Continue,
        }
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        let mut authority = "";
        let headers = self.get_http_request_headers();
        for (name, value) in &headers {
            debug!("#{} <- {}: {}", self.context_id, name, value);
            if name == "authority" {
                authority = value.as_str();
            }
        }
        self.set_http_response_header("x-hello", Some(&format!("Hello {}", authority)));
        self.set_http_response_header(
            "",
            Some(&format!(
                "xp3_istio_wasme_rust_hello_header #{}",
                self.context_id
            )),
        );
        Action::Continue
    }

    fn on_log(&mut self) {
        debug!("#{} completed.", self.context_id);
    }
}
