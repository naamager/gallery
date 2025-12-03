use actix_web::{web, Scope};
use crate::controllers::customer_controller;

pub fn customer_routes() -> Scope {
    web::scope("/customers")
        .service(customer_controller::get_customers)
        .service(customer_controller::get_customers_in_jerusalem)
        // .service(customer_controller::get_customers_by_id)
        .service(customer_controller::create_customer)
        .service(customer_controller::delete_customer)
        .service(customer_controller::update_customer)
}
