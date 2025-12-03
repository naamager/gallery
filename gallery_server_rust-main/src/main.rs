use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use crate::controllers::init_db;
use crate::routes::customers_routes::customer_routes;
use crate::routes::artists_routes::artist_routes;
use crate::routes::artworks_routes::artworks_routes; 
use crate::routes::artworks_in_order_routes::artworks_in_order_routes;
use crate::routes::orders_routes::orders_routes;

mod models;
mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let pool = init_db::init_db().await.expect("Failed to initialize database");
    
    println!("🚀 Server running at http://127.0.0.1:3007");
    println!("📊 SQLite database initialized at src/mydb.db");

    HttpServer::new(move || {
        let cors = Cors::permissive(); // Allow all origins for development
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .service(customer_routes())
            .service(artist_routes())
            .service(artworks_routes())
            .service(orders_routes())
            .service(artworks_in_order_routes())
    })
    .bind(("127.0.0.1", 3007))?
    .run()
    .await
}