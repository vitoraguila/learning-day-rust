use crate::models::customer::Customer;
use crate::models::customer::CustomerDocument;
use crate::models::customer::CustomerInput;

use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, DateTime, Document};
use mongodb::options::FindOneAndUpdateOptions;
use mongodb::options::FindOptions;
use mongodb::options::ReturnDocument;
use mongodb::Database;
use rocket::serde::json::Json;
use mongodb::error::Result as MongoResult;
use futures::stream::TryStreamExt;

pub async fn insert_customer(
    db: &Database,
    input: Json<CustomerInput>
) -> MongoResult<String>
{
    let collection = db.collection::<Document>("customer");
    let created_at: DateTime = DateTime::now();
    let updated_at: DateTime = created_at.clone();

    let res = collection.insert_one(
        doc!("name": input.name.clone(), "city": input.city.clone(), "createdAt": created_at, "updatedAt": updated_at), None).await.unwrap();
    Ok(res.inserted_id.to_string())

}

pub async fn find_customer_by_id(
    db: &Database,
    _id: ObjectId
) -> MongoResult<Option<Customer>>
{
    let collection = db.collection::<CustomerDocument>("customer");
    let res = collection.find_one(
        doc!{"_id": _id}, None).await.unwrap();
    
    if res.is_none(){
        return Ok(None);
    }

    let unwraped_doc = res.unwrap();

    let c_json = Customer{
        _id: unwraped_doc._id.to_string(),
        name: unwraped_doc.name.to_string(),
        city: unwraped_doc.city.to_string(),
        createdAt: unwraped_doc.createdAt.to_string(),
        updatedAt: unwraped_doc.updatedAt.to_string()
    };

    Ok(Some(c_json))
}

pub async fn find_customer(
    db: &Database,
    limit: i64,
    page: i64
) -> MongoResult<Vec<Customer>>
{
    let collection = db.collection::<CustomerDocument>("customer");
    let find_optins = FindOptions::builder().limit(limit).skip(u64::try_from((page-1)*limit).unwrap()).build();

    let mut cursor = collection.find(None, find_optins).await.unwrap();

    let mut customers: Vec<Customer> = vec![];

    while let Some(result) = cursor.try_next().await.unwrap(){
        let c_json = Customer{
            _id: result._id.to_string(),
            name: result.name.to_string(),
            city: result.city.to_string(),
            createdAt: result.createdAt.to_string(),
            updatedAt: result.updatedAt.to_string(),
        };
        customers.push(c_json);
    }
    Ok(customers)
}

pub async fn update_customer_by_id(
    db: &Database,
    _id: ObjectId,
    input: Json<CustomerInput>
) -> MongoResult<Option<Customer>>
{
    let collection = db.collection::<CustomerDocument>("customer");
    let options = FindOneAndUpdateOptions::builder().return_document(ReturnDocument::After).build();
    let updated_at = DateTime::now();
    let find_res = collection.find_one(
        doc!{"_id": _id}, None).await.unwrap();

    if find_res.is_none(){
        return Ok(None);
    }
    let created_at = find_res.unwrap().createdAt;
    let doc = collection.find_one_and_update(
        doc!{"_id": _id}, doc!{"$set": {"name": input.name.to_string(), "city": input.city.to_string(), "createdAt": created_at, "updatedAt": updated_at}}, options).await.unwrap();

        let unwraped_doc = doc.unwrap();

        let c_json = Customer{
            _id: unwraped_doc._id.to_string(),
            name: unwraped_doc.name.to_string(),
            city: unwraped_doc.city.to_string(),
            createdAt: unwraped_doc.createdAt.to_string(),
            updatedAt: unwraped_doc.updatedAt.to_string()
        };
    Ok(Some(c_json))
}

pub async fn delete_customer_by_id(
    db: &Database,
    _id: ObjectId
) -> MongoResult<Option<Customer>>
{
    let collection = db.collection::<CustomerDocument>("customer");
    let customer_doc = collection
        .find_one_and_delete(doc! {"_id":_id }, None)
        .await.unwrap();
    if customer_doc.is_none() {
        return Ok(None);
    }

    let unwraped_doc = customer_doc.unwrap();

        let c_json = Customer{
            _id: unwraped_doc._id.to_string(),
            name: unwraped_doc.name.to_string(),
            city: unwraped_doc.city.to_string(),
            createdAt: unwraped_doc.createdAt.to_string(),
            updatedAt: unwraped_doc.updatedAt.to_string()
        };
    Ok(Some(c_json))
}