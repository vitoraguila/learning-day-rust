use crate::models::customer::{Customer, CustomerDocument, CustomerInput};
use crate::db::customer;
use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
use rocket::State;

#[get("/customer?<limit>&<page>")]
pub async fn get_customers(
    db: &State<Database>,
    limit: Option<i64>,
    page: Option<i64>
) -> Result<Json<Vec<Customer>>, &'static str>
{
    let my_limit:i64 = limit.unwrap_or(12);
    let my_page:i64 = page.unwrap_or(12);

    match customer::find_customer(&db, my_limit, my_page).await{
        Ok(_doc)=>{
            Ok(Json(_doc))
        },
        Err(e)=>{
            println!("Error = {:?}",e);
            Err("Error Occured")
        }
    }
}

#[get("/customer/<_id>")]
pub async fn get_customer_by_id(
    db: &State<Database>,
    _id: String
) -> Result<Json<Customer>, &'static str>
{
    let oid = ObjectId::parse_str(_id);

    if oid.is_err(){
        return Err("Object ID Parse Failed")
    }

    match customer::find_customer_by_id(&db, oid.unwrap()).await{
        Ok(doc)=>{
            if doc.is_none(){
                return Err("Customer Not Found")
            }
            Ok(Json(doc.unwrap()))
        },
        Err(e)=>{
            println!("Error = {:?}",e);
            Err("Error Occured")
        }
    }
}

#[post("/customer", data="<input>")]
pub async fn post_customer(
    db: &State<Database>,
    input: Json<CustomerInput>
) -> Result<Json<String>, &'static str>

{
    match customer::insert_customer(&db, input).await{
        Ok(doc_id)=>{
            Ok(Json(doc_id))
            },
        Err(e)=>{
            println!("Error = {:?}",e);
            Err("Error Occured")
        }
    }
}


#[patch("/customer/<_id>", data="<input>")]
pub async fn patch_customer_by_id(
    db: &State<Database>,
    input: Json<CustomerInput>,
    _id: String
) -> Result<Json<Customer>, &'static str>
{
    let oid = ObjectId::parse_str(_id);
    if oid.is_err(){
        return Err("Object ID Parse Failed")
    }

    match customer::update_customer_by_id(&db, oid.unwrap(), input).await{
        Ok(doc)=>{
            if doc.is_none(){
                return Err("Customer Not Found")
            }
            Ok(Json(doc.unwrap()))
        },
        Err(e)=>{
            println!("Error = {:?}",e);
            Err("Error Occured")
        }
    }
}

#[delete("/customer/<_id>")]
pub async fn delete_customer_by_id(
    db: &State<Database>,
    _id: String
)-> Result<Json<Customer>, &'static str>
{
    let oid = ObjectId::parse_str(_id);
    if oid.is_err(){
        return Err("Object ID Parse Failed")
    }

    match customer::delete_customer_by_id(&db, oid.unwrap()).await{
        Ok(doc)=>{
            if doc.is_none(){
                return Err("Customer Not Found")
            }
            Ok(Json(doc.unwrap()))
        },
        Err(e)=>{
            println!("Error = {:?}",e);
            Err("Error Occured")
        }
    }
}