use rocket::serde::json::Json;
use crate::models::response::MessageResponse;
pub mod customer;

#[get("/")]
pub fn index() -> Json<MessageResponse> {
    Json(MessageResponse{
        message: "Welcome to Final Project".to_string()
    })
}