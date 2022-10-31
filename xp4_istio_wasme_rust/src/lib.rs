use config::CsClientConfig;
use log::debug;
mod config;

use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(HttpHeadersRoot {
            config: config::CsClientConfig::default() })
    });
}}

struct HttpHeadersRoot {
    config: config::CsClientConfig,
}

impl Context for HttpHeadersRoot {}

impl RootContext for HttpHeadersRoot {
    fn on_configure(&mut self, _plugin_config_size: usize) -> bool {
        if let Some(config_bytes) = self.get_plugin_configuration() {
            if let Some(config) = CsClientConfig::new_from_config_bytes(config_bytes) {
                debug!("[on_configure] Loaded config: {:?}", config);
                self.config = config;
                return true;
            }
        }
        false
    }

    fn get_type(&self) -> Option<ContextType> {
        Some(ContextType::HttpContext)
    }

    fn create_http_context(&self, context_id: u32) -> Option<Box<dyn HttpContext>> {
        Some(Box::new(HttpHeaders {
            context_id,
            curr_shared_key: None,
            prev_shared_key: None,
        }))
    }
}

const SHARED_KEY_HEADER: &str = "x-sk";

struct HttpHeaders {
    context_id: u32,
    curr_shared_key: Option<String>,
    prev_shared_key: Option<String>,
}

impl Context for HttpHeaders {}

impl HttpContext for HttpHeaders {
    fn on_http_request_headers(&mut self, _hdrs_cnt: usize, _eos: bool) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            debug!("#{} -> {}: {}", self.context_id, name, value);
        }

        if let Some(sk) = self.get_http_request_header(SHARED_KEY_HEADER) {
            self.curr_shared_key = Some(sk);
            // cas (abreviation for compare-and-switch) is used when updating the value.
            let (prev_sk_opt, cas) = &self.get_shared_data(SHARED_KEY_HEADER);

            debug!(
                "#{} -> received {}={:?}",
                self.context_id, SHARED_KEY_HEADER, prev_sk_opt
            );

            match prev_sk_opt {
                Some(prev_sk) => {
                    self.prev_shared_key = Some(
                        std::str::from_utf8(prev_sk.clone().as_slice())
                            .unwrap_or_default()
                            .to_string(),
                    )
                }
                None => (),
            };

            match &self.set_shared_data(
                SHARED_KEY_HEADER,
                Some(self.curr_shared_key.as_ref().unwrap().as_bytes()),
                *cas,
            ) {
                Ok(()) => debug!(
                    "#{} -> set {}={:?}",
                    self.context_id, SHARED_KEY_HEADER, &self.curr_shared_key
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
        let res_sk = match &self.curr_shared_key {
            Some(curr_sk) => match &self.prev_shared_key {
                Some(prev_sk) => Some(format!("{:?}|{:?}", curr_sk, prev_sk)),
                None => Some(curr_sk.clone()),
            },
            None => match &self.prev_shared_key {
                Some(prev_sk) => Some(format!("{:?}", prev_sk)),
                None => None,
            },
        };
        self.set_http_response_header(SHARED_KEY_HEADER, res_sk.as_deref());

        self.set_http_response_header(
            "x-sk-processor",
            Some(&format!("xp4_istio_wasme_rust #{}", self.context_id)),
        );

        let headers = self.get_http_request_headers();
        for (name, value) in &headers {
            debug!("#{} <- {}: {}", self.context_id, name, value);
        }

        Action::Continue
    }

    fn on_log(&mut self) {
        debug!("#{} completed.", self.context_id);
    }
}
