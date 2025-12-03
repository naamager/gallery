use actix_web::{web, Scope};
use crate::controllers::artist_controller;

pub fn artist_routes() -> Scope {
    web::scope("/artists")
        .service(artist_controller::get_artists)
        .service(artist_controller::get_artists_born_after_1980)
        //.service(artist_controller::get_artist_by_id)
        .service(artist_controller::create_artist)  
        .service(artist_controller::delete_artist)
        .service(artist_controller::update_artist)
}