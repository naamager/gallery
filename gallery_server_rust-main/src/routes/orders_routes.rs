use actix_web::{web, Scope};
use crate::controllers::order_controller::{get_orders, create_order, get_order_by_id, update_order, delete_order, get_orders_after_2025_01_01, get_detailed_orders};

pub fn orders_routes() -> Scope {
    web::scope("/orders")
        .service(get_orders)
        .service(create_order)
        .service(get_detailed_orders)
        .service(get_orders_after_2025_01_01)
        .service(get_order_by_id)
        .service(update_order)
        .service(delete_order)
}