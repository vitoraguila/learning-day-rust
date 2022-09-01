use rocket_okapi::JsonSchema;
use serde::{Serialize, Deserialize};

#[derive(Responder, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MessageResponse{
    pub message: String
}