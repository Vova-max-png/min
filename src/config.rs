use std::io::Error;
use std::{ fmt, fmt::Display };
use std::fs;

use serde::{Deserialize, Serialize};

use crate::max_provider::UserAgent;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserAuthAgent {
    compatibility_token: String,
    platform: String,
    rendering_engine: String,
    compatibility_tags: String,
    browser_identity: String,
    safari_compatibility: String
}

impl UserAuthAgent {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}

impl Default for UserAuthAgent {
    fn default() -> Self {
        Self {
            compatibility_token: "Mozilla/5.0".to_string(),
            platform: "(Windows NT 10.0; Win64; x64)".to_string(),
            rendering_engine: "AppleWebKit/537.36".to_string(),
            compatibility_tags: "(KHTML, like Gecko)".to_string(),
            browser_identity: "Chrome/142.0.0.0".to_string(),
            safari_compatibility: "Safari/537.36".to_string()
        }
    }
}

impl Display for UserAuthAgent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {} {}", 
            self.compatibility_token,
            self.platform,
            self.rendering_engine,
            self.compatibility_tags,
            self.browser_identity,
            self.safari_compatibility
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Headers {
    host: String,
    #[serde(rename = "Accept-Encoding")]
    accept_encoding: String,
    #[serde(rename = "Accept-Language")]
    accept_language: String,
    connection: String,
    origin: String,
    pragma: String,
    #[serde(rename = "Sec-Websocket-Extension")]
    sec_websocket_extension: String,
    #[serde(rename = "Sec-Websocket-Key")]
    sec_websocket_key: String,
    #[serde(rename = "Sec-Websocket-Version")]
    sec_websocket_version: String,
    upgrade: String,
    #[serde(rename = "User-Agent")]
    user_agent: String
}

impl Default for Headers {
    fn default() -> Self {
        Self {
            host: "ws-api.oneme.ru".to_string(),
            accept_encoding: "gzip, deflate, br, zstd".to_string(),
            accept_language: "en-US,en;q=0.9".to_string(),
            connection: "Upgrade".to_string(),
            origin: "https://web.max.ru".to_string(),
            pragma: "no-cache".to_string(),
            sec_websocket_extension: "permessage-deflate; client_max_window_bits".to_string(),
            sec_websocket_key: "MEBa2ZnucwlWNZrrLRbmIQ".to_string(),
            sec_websocket_version: "13".to_string(),
            upgrade: "websocket".to_string(),
            user_agent: UserAuthAgent::new().to_string()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigParser {
    pub headers: Headers,
    pub max_agent: UserAgent
}

impl ConfigParser {
    pub fn parse_config_file(file_name: &str) -> Result<Self, Error> {
        let raw_config = fs::read_to_string(file_name)?;
        println!("Raw config: {}", raw_config);

        let config_instance: ConfigParser = serde_json::from_str(&raw_config)?;
        println!("Parsed config: {:#?}", config_instance);

        Ok(config_instance)
    }
}