use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool;
use crate::models::artwork_in_order::{ArtworkInOrder};
use uuid::Uuid;

pub async fn init_artworks_in_order_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("📋 Creating artworks_in_order table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS artworks_in_order (
            id_artwork_in_order TEXT PRIMARY KEY NOT NULL,
            id_order TEXT NOT NULL,
            id_artwork TEXT NOT NULL,
            amount INTEGER NOT NULL,
            FOREIGN KEY (id_order) REFERENCES orders(id_order),
            FOREIGN KEY (id_artwork) REFERENCES artworks(id_artwork)
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("✅ artworks_in_order table ready");
    
    Ok(())
}

#[get("/")]
pub async fn get_artworks_in_order(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, ArtworkInOrder>(
        r#"
        SELECT id_artwork_in_order, id_order, id_artwork, amount
        FROM artworks_in_order
        ORDER BY id_order, id_artwork
        "#
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(artworks_in_order) => HttpResponse::Ok().json(artworks_in_order),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[post("/")]
pub async fn create_artwork_in_order(pool: web::Data<SqlitePool>, artwork_in_order: web::Json<ArtworkInOrder>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    
    match sqlx::query("INSERT INTO artworks_in_order (id_artwork_in_order, id_order, id_artwork, amount) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&artwork_in_order.id_order)
        .bind(&artwork_in_order.id_artwork)
        .bind(&artwork_in_order.amount)
        .execute(&**pool)
        .await
    {
        Ok(_result) => {
            let new_artwork_in_order = ArtworkInOrder {
                id_artwork_in_order: id,
                id_order: artwork_in_order.id_order.clone(),
                id_artwork: artwork_in_order.id_artwork.clone(),
                amount: artwork_in_order.amount,
            };
            HttpResponse::Created().json(new_artwork_in_order)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{id_artwork_in_order}")]
pub async fn delete_artwork_in_order(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let id_artwork_in_order = path.into_inner();
    
    match sqlx::query("DELETE FROM artworks_in_order WHERE id_artwork_in_order = ?")
        .bind(&id_artwork_in_order)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Artwork in order deleted successfully")
            } else {
                HttpResponse::NotFound().body("Artwork in order not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{id_artwork_in_order}")]
pub async fn update_artwork_in_order(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
    updated: web::Json<ArtworkInOrder>,
) -> impl Responder {
    let id_artwork_in_order = path.into_inner();
    
    match sqlx::query("SELECT id_artwork_in_order FROM artworks_in_order WHERE id_artwork_in_order = ?")
        .bind(&id_artwork_in_order)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query("UPDATE artworks_in_order SET id_order = ?, id_artwork = ?, amount = ? WHERE id_artwork_in_order = ?")
                .bind(&updated.id_order)
                .bind(&updated.id_artwork)
                .bind(&updated.amount)
                .bind(&id_artwork_in_order)
                .execute(&**pool)
                .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        let updated_artwork_in_order = ArtworkInOrder {
                            id_artwork_in_order: id_artwork_in_order.clone(),
                            id_order: updated.id_order.clone(),
                            id_artwork: updated.id_artwork.clone(),
                            amount: updated.amount,
                        };
                        HttpResponse::Ok().json(updated_artwork_in_order)
                    } else {
                        HttpResponse::NotFound().body("Artwork in order not found")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Artwork in order with provided ID does not exist"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
