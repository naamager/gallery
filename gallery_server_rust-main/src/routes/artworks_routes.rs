use actix_web::{web, Scope};
use crate::controllers::artwork_controller;

pub fn artworks_routes() -> Scope {
    web::scope("/artworks")
        .service(artwork_controller::get_all_artworks)
        .service(artwork_controller::get_artwork_by_id)
        .service(artwork_controller::get_artworks_by_type)
        .service(artwork_controller::create_artwork)  
        .service(artwork_controller::delete_artwork)
        .service(artwork_controller::update_artwork)
}
