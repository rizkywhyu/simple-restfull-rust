use actix_web::{post, web::{self}, HttpResponse};
use model::{modules::{sample_model::RequestSendMessage}};


#[post("/send")]
pub async fn send_msg(_opt:web::Json<RequestSendMessage>) -> HttpResponse {
    let response = model::modules::sample_model::ResponseSendMessage::send_msg(_opt);
    response.await
}