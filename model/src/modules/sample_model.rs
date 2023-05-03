use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestSendMessage {
    pub platform: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub r#type: Option<String>,
    pub templateName: Option<String>,
    pub templateLang: Option<String>,
    pub headerType: Option<String>,
    pub header: Option<String>,
    pub templateData: Vec<String>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSendMessage {
    pub status: Option<i32>,
    pub errorCode: Option<i32>,
    pub message: Option<String>,
    pub data: Option<Vec<String>>
}

impl ResponseSendMessage {
    pub async fn send_msg(_opt:web::Json<RequestSendMessage>) -> HttpResponse {
        let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build().unwrap();

        let url = env::var("URL").unwrap_or("".to_string());
        let bearer = env::var("AUTH_BEARER").unwrap_or("".to_string());

        let mut payload = serde_json::json!({
            "platform": _opt.platform,
            "from": _opt.from,
            "to": _opt.to,
            "type": _opt.r#type ,
            "templateName": _opt.templateName,
            "templateLang": _opt.templateLang,
            "headerType": _opt.headerType,
            "header": _opt.header,
            "templateData": _opt.templateData
        });

        let response = client.post(url)
        .header("Content-Type","application/json")
        .bearer_auth(bearer)
        .header("Accept","application/json")
        .body(payload.to_string())
        .send()
        .await.unwrap();

        match response.status() {
            reqwest::StatusCode::OK => {
                match response.json::<ResponseSendMessage>().await {
                    Ok(parsed) => HttpResponse::Ok().json(parsed),
                    Err(_) => HttpResponse::BadRequest().json("a")
                    
                }
            },
            reqwest::StatusCode::BAD_REQUEST => {
                HttpResponse::BadRequest().json("Badrequest")
            }
            _ => {
                panic!("Something wrong!!");
            }
        }
    }
}