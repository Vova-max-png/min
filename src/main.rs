use std::collections::HashMap;

use tokio::net::{TcpSocket, TcpStream};
use tokio_tungstenite::{connect_async, tungstenite::{client::IntoClientRequest, handshake::client::Request, protocol::Message}};
use futures_util::sink::SinkExt;

mod provider;
use provider::{ Provider, Data };

use crate::provider::UserAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token =  "An_Sx6HQ9HDiosqclOvmtrXJjp39jF4tyO3OPzfHXeRLT5T2osJz70TmnCCvE7jadYAwTD0nSzEmQ__QlaQjlndRekx74h8d8yOcLzrS6qrQmRey7yeYU8-roYfBVgh5wzTErjWt7cLIqMfGKXpWvz8-7Obw6TyBEfrXtBJuvfHdtIk7qv90w2lIhWDIQYjfpdfkg-8LrQV-ndkJj6lwD4rgypCXdF4KGlecl0ZeNBYeap32ZxG1KABw8BmHG4r1wWrLvJ_AW0GTXhY-83Ev6GBEFEG3ngTnCzioc3GOEVPbqlBnz1XaaGmfKq1BugKgMcrKVF_4kB6O8zatRr0Iun38V1JjUSLB4chy6jW8Rg3dUTjgALgwnNK4KM3jUuIK29TDSIw7wH513RGBgV4JKLKsfV0dJKo5yR_2joWjsH2cWZgTntbPHe9c4XYvT0t6-ORwFpHiTMpHIpqlgfruntAAiEUV7g6WxjuBbrjZ-i1EMsxAkphjBdWQgg5U_MgrctbsnD7x5DiFj93fVq8BmrFfgBdfV9BKy_UiA-N1m_HChuNQhugJjMwW-NJwsACyltYWipvzkYxSfu_W9Ozt5pGKTjJTs4iyKzbRDjr-zhQ3_bDnPLc9dILdfLPGdpQtpzPduIIMLdcI72NqN7FmvU01smzp696336NQl5ET3_UBLODpEHVcCCXQBuvoWqopylH2b7M";

    let mut headers = HashMap::new();
        headers.insert("Host", "ws-api.oneme.ru");
        headers.insert("Accept-Encoding", "gzip, deflate, br, zstd");
        headers.insert("Accept-Language", "en-US,en;q=0.9");
        headers.insert("Connection", "Upgrade");
        headers.insert("Origin", "https://web.max.ru");
        headers.insert("Pragma", "no-cache");
        headers.insert("Sec-Websocket-Extension", "permessage-deflate; client_max_window_bits");
        headers.insert("Sec-Websocket-Key", "MEBa2ZnucwlWNZrrLRbmIQ==");
        headers.insert("Sec-Websocket-Version", "13");
        headers.insert("Upgrade", "websocket");
        headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36");
    
    let provider = Provider::new(token.to_string(), headers, "wss://ws-api.oneme.ru/websocket".to_string()).await?;

    // let request = Request::builder()
    //     .uri("wss://ws-api.oneme.ru/websocket")
    //     .header("Host", "ws-api.oneme.ru")
    //     .header("Accept-Encoding", "gzip, deflate, br, zstd")
    //     .header("Accept-Language", "en-US,en;q=0.9")
    //     .header("Connection", "Upgrade")
    //     .header("Origin", "https://web.max.ru")
    //     .header("Pragma", "no-cache")
    //     .header("Sec-Websocket-Extension", "permessage-deflate; client_max_window_bits")
    //     .header("Sec-Websocket-Key", "MEBa2ZnucwlWNZrrLRbmIQ==")
    //     .header("Sec-Websocket-Version", "13")
    //     .header("Upgrade", "websocket")
    //     .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36")
    //     .body(())?;

    // let (mut stream, response) = connect_async(request).await?;

    // println!("Connected!");

    // stream.send(Message::Text("{\"ver\":11,\"cmd\":0,\"seq\":0,\"opcode\":6,\"payload\":{\"userAgent\":{\"deviceType\":\"WEB\",\"locale\":\"ru\",\"deviceLocale\":\"en\",\"osVersion\":\"Windows\",\"deviceName\":\"Chrome\",\"headerUserAgent\":\"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36\",\"appVersion\":\"25.12.11\",\"screen\":\"1080x1920 1.0x\",\"timezone\":\"Europe/Moscow\"},\"deviceId\":\"13977301-4cfd-4cb4-98b6-3536e0744015\"}}".into())).await?;
    // println!("{:#?}", response);

    let user_agent = UserAgent {
        app_version: "25.12.13".to_string(),
        device_locale: "en".to_string(),
        device_name: "Chrome".to_string(),
        device_type: "WEB".to_string(),
        header_user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/142.0.0.0 Safari/537.36".to_string(),
        locale: "ru".to_string(),
        os_version: "Linux".to_string(),
        screen: "1080x1920 1.0x".to_string(),
        timezone: "Europe/Saints-Petersburg".to_string()
    };

    let user_agent_data = Data {
        device_id: Some("13977301-4cfd-4cb4-98b6-3536e0744015".to_string()),
        user_agent: Some(user_agent),
        ..Default::default()
    };

    let auth_data = Data {
        chats_count: Some(40),
        chats_sync: Some(0),
        contacts_sync: Some(0),
        drafts_sync: Some(0),
        interactive: Some(true),
        presence_sync: Some(-1),
        token: Some(token.to_string()),
        ..Default::default()
    };

    // Send UserAgent(max be like)
    provider.send_data(user_agent_data, 6).await?.send_data(auth_data, 19).await?.handle_messages().await;
    
    // println!("{:#?}", response.status());
    // Message::Text("{\"ver\":11,\"cmd\":0,\"seq\":1,\"opcode\":19,\"payload\":{\"interactive\":true,\"token\":\"An_Sx6HQ9HDiosqclOvmtrXJjp39jF4tyO3OPzfHXeRLT5T2osJz70TmnCCvE7jadYAwTD0nSzEmQ__QlaQjlndRekx74h8d8yOcLzrS6qrQmRey7yeYU8-roYfBVgh5wzTErjWt7cLIqMfGKXpWvz8-7Obw6TyBEfrXtBJuvfHdtIk7qv90w2lIhWDIQYjfpdfkg-8LrQV-ndkJj6lwD4rgypCXdF4KGlecl0ZeNBYeap32ZxG1KABw8BmHG4r1wWrLvJ_AW0GTXhY-83Ev6GBEFEG3ngTnCzioc3GOEVPbqlBnz1XaaGmfKq1BugKgMcrKVF_4kB6O8zatRr0Iun38V1JjUSLB4chy6jW8Rg3dUTjgALgwnNK4KM3jUuIK29TDSIw7wH513RGBgV4JKLKsfV0dJKo5yR_2joWjsH2cWZgTntbPHe9c4XYvT0t6-ORwFpHiTMpHIpqlgfruntAAiEUV7g6WxjuBbrjZ-i1EMsxAkphjBdWQgg5U_MgrctbsnD7x5DiFj93fVq8BmrFfgBdfV9BKy_UiA-N1m_HChuNQhugJjMwW-NJwsACyltYWipvzkYxSfu_W9Ozt5pGKTjJTs4iyKzbRDjr-zhQ3_bDnPLc9dILdfLPGdpQtpzPduIIMLdcI72NqN7FmvU01smzp696336NQl5ET3_UBLODpEHVcCCXQBuvoWqopylH2b7M\",\"chatsCount\":40,\"chatsSync\":0,\"contactsSync\":0,\"presenceSync\":-1,\"draftsSync\":0}}".into())

    Ok(())
}
