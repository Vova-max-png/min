use std::{collections::HashMap, io::Error};

use serde::{Deserialize, Serialize};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::{ http::{Request, Response}, protocol::Message }};
use futures_util::{StreamExt, sink::SinkExt, stream::{SplitSink, SplitStream}};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserAgent {
    pub app_version: String,
    pub device_locale: String,
    pub device_name: String,
    pub device_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_user_agent: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaxSelfNames {
    name: String,
    first_name: String,
    last_name: String,
    #[serde(rename = "type")]
    typ: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaxSelfContact {
    account_status: i8,
    base_url: String,
    names:Vec<MaxSelfNames>,
    phone: i64,
    options: Vec<String>,
    photo_id: i64,
    update_time: usize,
    id: i64,
    base_raw_url: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaxSelfProfile {
    profile_options: Vec<i8>,
    contact: MaxSelfContact
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct MaxChatOptions {
    sign_admin: Option<bool>,
    official: Option<bool>,
    message_copy_not_allowed: Option<bool>,
    only_owner_can_change_icon_title: Option<bool>,
    only_admin_can_add_member: Option<bool>,
    only_admin_can_call: Option<bool>,
    sent_by_phone: Option<bool>,
    content_level_chat: Option<bool>,
    a_plus_channel: Option<bool>,
    all_can_pin_message: Option<bool>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastMessageElement {
    #[serde(rename = "type")]
    typ: String,
    length: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastMessage {
    elements: Vec<LastMessageElement>,
    options: i64,
    id: i64,
    time: usize,
    text: String,
    #[serde(rename = "type")]
    typ: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaxChat {
    participants_count: Option<i64>,
    access: Option<String>,
    invited_by: Option<i64>,
    base_raw_icon_url: Option<String>,
    link: Option<String>,
    description: Option<String>,
    #[serde(rename = "type")]
    typ: String,
    title: Option<String>,
    last_fire_delayed_error_time: i64,
    last_delayed_update_time: i64,
    new_messages: Option<i64>,
    options: Option<MaxChatOptions>,
    modified: usize,
    id: i64,
    owner: i64,
    join_time: usize,
    created: usize,
    restrictions: Option<i64>,
    last_event_time: usize,
    messages_count: Option<i64>,
    base_icon_url: Option<String>,
    status: String,
    cid: Option<i64>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContactName {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    first_name: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    account_status: i64,
    names: Vec<ContactName>,
    update_time: usize,
    id: i64
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MaxResponse {
    video_chat_history: bool,
    profile: MaxSelfProfile,
    chats: Vec<MaxChat>,
    contacts: Vec<Contact>
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseState {
    cmd: i8,
    opcode: i64,
    seq: i8,
    ver: i8,
    pub payload: MaxResponse
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResponseHeaders {
    cmd: i8,
    opcode: i64,
    seq: i8,
    ver: i8
}

#[derive(Serialize, Deserialize, Clone)]
pub struct RequestState {
    cmd: i8,
    opcode:i64,
    seq: i8,
    ver: i8,
    pub payload: Option<Data>
}

impl RequestState {
    pub fn new() -> Self {
        Self {
            cmd: 0,
            ver: 11,
            opcode: 0,
            seq: 0,
            payload: None
        }
    }

    pub fn set_opcode(&mut self, opcode: i64) {
        self.opcode = opcode;
    }

    pub fn increase_seq(&mut self) {
        self.seq += 1;
    }
}

pub struct Provider {
    read: SplitStream<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>,
    write: SplitSink<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>, Message>,
    response: Response<Option<Vec<u8>>>,
    state: RequestState,
    full_data: Option<ResponseState>
}

impl Provider {
    pub async fn new(headers: String, uri: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut request = Request::builder();
        let map: HashMap<&str, &str> = serde_json::from_str(&headers)?;
        request = request.uri(uri);
        for (k,v) in map {
            request = request.header(k, v);
        }

        let (stream, response) = connect_async(request.body(())?).await?;

        let (write, read) = stream.split();

        let state = RequestState::new();

        Ok(Self {
            read,
            write,
            response,
            state,
            full_data: None
        })
    }

    pub async fn send_data(mut self, data: Data, opcode: i64) -> Result<Self, Box<dyn std::error::Error>> {
        self.state.set_opcode(opcode);
        let mut state_copy = self.state.clone();
        state_copy.payload = Some(data);
        let raw_data = serde_json::to_string(&state_copy)?;
        println!("Data to send: {}", raw_data);
        self.write.send(Message::Text(raw_data)).await?;

        println!("Response: {:#?}", self.response);

        self.state.increase_seq();

        Ok(self)
    }

    pub async fn handle_messages(&mut self) -> Result<(), Error> {
        loop {
            match self.read.next().await {
                Some(Ok(Message::Text(text))) => {
                    println!("Received: {}", text);
                    let headers: ResponseHeaders = serde_json::from_str(&text).unwrap();
                    if headers.opcode == 19 {
                        let response: ResponseState = serde_json::from_str(&text).unwrap();
                        self.full_data = Some(response.clone());
                        println!("{:#?}", response);
                    }
                },
                _ => {}
            }
        }
    }
}