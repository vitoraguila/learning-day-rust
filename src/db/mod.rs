use rocket::fairing::AdHoc;
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub mod customer;
mod consts;

pub fn init() -> AdHoc {
    AdHoc::on_ignite("Connecting to MongoDB........",|rocket| async{
        match connect().await{
            Ok(db)=> rocket.manage(db),
            Err(e)=>  {
                panic!("Cannot connect to database = {:?}", e)
            }
        }
    })
}

async fn connect() -> mongodb::error::Result<Database> {
    let mongo_uri = consts::MONGO_URI;
    let mongo_db = consts::MONGO_DB_NAME;

    let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let database = client.database(mongo_db);

    println!("MongoDB Connected......!");
    Ok(database)
}