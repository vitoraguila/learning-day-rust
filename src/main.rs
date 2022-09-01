#[macro_use]
extern crate rocket;

mod db;
mod models;
mod routes;

#[launch]
fn rocket()-> _ {
    rocket::build().attach(db::init()).mount("/",
    routes![
        routes::index,
        routes::customer::get_customers,
        routes::customer::get_customer_by_id,
        routes::customer::post_customer,
        routes::customer::patch_customer_by_id,
        routes::customer::delete_customer_by_id
    ]
)
}