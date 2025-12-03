use actix_web::{web, Scope};
use crate::controllers::artwork_in_order_controller;

pub fn artworks_in_order_routes() -> Scope {
    web::scope("/artworks_in_order")
        .service(artwork_in_order_controller::get_artworks_in_order)
        .service(artwork_in_order_controller::create_artwork_in_order)
        .service(artwork_in_order_controller::update_artwork_in_order)  
        .service(artwork_in_order_controller::delete_artwork_in_order)
}