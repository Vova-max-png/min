use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{ http::{Request, Response}, protocol::Message }};
use futures_util::{StreamExt, sink::SinkExt};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub app_version: String,
    pub device_locale: String,
    pub device_name: String,
    pub device_type: String,
    pub header_user_agent: String,
    pub locale: String,
    pub os_version: String,
    pub screen: String,
    pub timezone: String
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chats_sync: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drafts_sync: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_sync: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts_sync: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<UserAgent>
}

impl Default for Data {
    fn default() -> Self {
        Data {
            interactive: None,
            chats_count: None,
            chats_sync: None,
            drafts_sync: None,
            presence_sync: None,
            contacts_sync: None,
            token: None,
            device_id: None,
            user_agent: None
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestState {
    cmd: i8,
    opcode: i16,
    seq: i8,
    ver: i8,
    pub payload: Option<Data>
}

impl RequestState {
    pub fn new(opcode: i16) -> Self {
        Self {
            cmd: 0,
            ver: 11,
            opcode,
            seq: 0,
            payload: None
        }
    }

    pub fn increase_seq(mut self) -> Self {
        self.seq += 1;

        self
    }

    pub fn set_opcode(mut self, new_opcode: i16) -> Self {
        self.opcode = new_opcode;

        self
    }
}

pub struct Provider {
    token: String,
    stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    response: Response<Option<Vec<u8>>>,
    state: RequestState
}

impl Provider {
    pub async fn new(token: String, headers: HashMap<&str, &str>, uri: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut request = Request::builder();
        request = request.uri(uri);
        for (k,v) in headers {
            request = request.header(k, v);
        }

        let (stream, response) = connect_async(request.body(())?).await?;

        let state = RequestState::new(6);

        Ok(Self {
            token,
            stream,
            response,
            state
        })
    }

    pub async fn send_data(mut self, data: Data, opcode: i16) -> Result<Self, Box<dyn std::error::Error>> {
        self.state.opcode = opcode;
        let mut state = self.state.clone();
        state.payload = Some(data);
        let raw_data = serde_json::to_string(&state)?;
        println!("Data to send: {}", raw_data);
        self.stream.send(Message::Text(raw_data)).await?;

        println!("Response: {:#?}", self.response);

        self.state.seq += 1;

        Ok(self)
    }

    pub async fn handle_messages(&mut self) {
        loop {
            match self.stream.next().await {
                Some(Ok(Message::Text(text))) => {
                    println!("Received: {}", text);
                },
                _ => {}
            }
        }
    }
}