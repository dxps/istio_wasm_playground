use log::{error, info};
use serde::{Deserialize, Serialize};

/// Cookie Svc Client Config.
/// Its value(s) are taken loaded from `pluginConfig` element.
/// See [wasm-plugin](https://istio.io/latest/docs/reference/config/proxy_extensions/wasm-plugin/) page for details.
#[derive(Serialize, Deserialize, Debug)]
pub struct CsClientConfig {
    cs_endpoint: String,
}

impl CsClientConfig {
    pub fn new_from_config_bytes(bytes: Vec<u8>) -> Option<Self> {
        match serde_json::from_slice::<CsClientConfig>(&bytes) {
            Ok(config) => {
                info!("Successfully loaded the plugin config.");
                Some(config)
            }
            Err(err) => {
                error!("Unable to load (deserialize) the config. Details: {}", err);
                None
            }
        }
    }
}

impl Default for CsClientConfig {
    fn default() -> Self {
        Self {
            cs_endpoint: Default::default(),
        }
    }
}
