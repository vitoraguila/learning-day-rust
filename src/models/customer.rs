use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDocument {
    pub _id: ObjectId,
    pub name: String,
    pub city: String,
    pub createdAt: DateTime,
    pub updatedAt: DateTime
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub _id: String,
    pub name: String,
    pub city: String,
    pub createdAt: String,
    pub updatedAt: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerInput {
    pub name: String,
    pub city: String,
    
}