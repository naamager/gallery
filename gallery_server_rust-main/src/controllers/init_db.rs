use sqlx::{self, SqlitePool};
use crate::controllers::customer_controller;
use crate::controllers::artist_controller;
use crate::controllers::artwork_controller;
use crate::controllers::order_controller;
use crate::controllers::artwork_in_order_controller;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;

    sqlx::query("PRAGMA foreign_keys = ON;").execute(&pool).await?;

    customer_controller::init_customers_table(&pool).await?;
    artist_controller::init_artists_table(&pool).await?;
    artwork_controller::init_artwork_table(&pool).await?;
    order_controller::init_orders_table(&pool).await?;
    artwork_in_order_controller::init_artworks_in_order_table(&pool).await?;

    
    Ok(pool)
}