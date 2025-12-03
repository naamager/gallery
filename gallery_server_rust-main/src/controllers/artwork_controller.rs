use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{sqlite::SqlitePool, Error};
use uuid::Uuid;
use crate::models::artwork::Artwork;

pub async fn init_artwork_table(pool: &SqlitePool) -> Result<(), Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ARTWORKS (
            id_artwork TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            year_created INTEGER NOT NULL,
            price REAL NOT NULL,
            id_artist TEXT NOT NULL,
            art_type TEXT NOT NULL,
            FOREIGN KEY (id_artist) REFERENCES artists(artist_id) ON DELETE CASCADE
        );
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[get("/")]
pub async fn get_all_artworks(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Artwork>("SELECT * FROM ARTWORKS")
        .fetch_all(&**pool)
        .await
    {
        Ok(artworks) => HttpResponse::Ok().json(artworks),
        Err(e) => {
            eprintln!("Failed to fetch artworks: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[get("/{id}")]
pub async fn get_artwork_by_id(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let id_artwork = path.into_inner();
    match sqlx::query_as::<_, Artwork>("SELECT * FROM ARTWORKS WHERE id_artwork = ?")
        .bind(&id_artwork)
        .fetch_one(&**pool)
        .await
    {
        Ok(artwork) => HttpResponse::Ok().json(artwork),
        Err(Error::RowNotFound) => HttpResponse::NotFound().body(format!("Artwork with id {} not found", id_artwork)),
        Err(e) => {
            eprintln!("Failed to fetch artwork by id {}: {}", id_artwork, e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[post("/")]
pub async fn create_artwork(pool: web::Data<SqlitePool>, mut artwork: web::Json<Artwork>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    artwork.id_artwork = Some(id.clone());
    let result = sqlx::query(
        r#"
        INSERT INTO artworks (id_artwork, title, description, year_created, price, id_artist, art_type)
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(artwork.id_artwork.as_ref().unwrap())
    .bind(&artwork.title)
    .bind(&artwork.description)
    .bind(&artwork.year_created)
    .bind(&artwork.price)
    .bind(&artwork.id_artist)
    .bind(&artwork.art_type)
    .execute(&**pool)
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(Artwork {
            id_artwork: Some(id.clone()), // Use the newly generated id
            title: artwork.title.clone(),
            description: artwork.description.clone(),
            year_created: artwork.year_created,
            price: artwork.price,
            id_artist: artwork.id_artist.clone(),
            art_type: artwork.art_type.clone(),
        }),
        Err(e) => {
            eprintln!("Failed to create artwork: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[put("/{id}")]
pub async fn update_artwork(pool: web::Data<SqlitePool>, path: web::Path<String>, artwork: web::Json<Artwork>) -> impl Responder {
    let id_artwork = path.into_inner();
    let result = sqlx::query(
        r#"
        UPDATE ARTWORKS SET title = ?, description = ?, year_created = ?, price = ?, id_artist = ?, art_type = ? WHERE id_artwork = ?
        "#
    )
    .bind(&artwork.title)
    .bind(&artwork.description)
    .bind(&artwork.year_created)
    .bind(&artwork.price)
    .bind(&artwork.id_artist)
    .bind(&artwork.art_type)
    .bind(&id_artwork)
    .execute(&**pool)
    .await;

    match result {
        Ok(res) => {
            if res.rows_affected() > 0 {
                HttpResponse::Ok().body(format!("Artwork with id {} updated successfully", id_artwork))
            } else {
                HttpResponse::NotFound().body(format!("Artwork with id {} not found", id_artwork))
            }
        },
        Err(e) => {
            eprintln!("Failed to update artwork with id {}: {}", id_artwork, e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[delete("/{id}")]
pub async fn delete_artwork(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let id_artwork = path.into_inner();

    // Delete related artworks in order first
    match sqlx::query("DELETE FROM artworks_in_order WHERE id_artwork = ?")
        .bind(&id_artwork)
        .execute(&**pool)
        .await
    {
        Ok(_) => {
            // Now delete the artwork
            match sqlx::query("DELETE FROM ARTWORKS WHERE id_artwork = ?")
                .bind(&id_artwork)
                .execute(&**pool)
                .await
            {
                Ok(res) => {
                    if res.rows_affected() > 0 {
                        HttpResponse::Ok().body(format!("Artwork with id {} deleted successfully", id_artwork))
                    } else {
                        HttpResponse::NotFound().body(format!("Artwork with id {} not found", id_artwork))
                    }
                },
                Err(e) => {
                    eprintln!("Failed to delete artwork with id {}: {}", id_artwork, e);
                    HttpResponse::InternalServerError().body(format!("Database error: {}", e))
                }
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error while deleting artworks in order: {}", e)),
    }
}

#[get("/type/{art_type}")]
pub async fn get_artworks_by_type(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let art_type = path.into_inner();
    match sqlx::query_as::<_, Artwork>("SELECT * FROM ARTWORKS WHERE art_type = ?")
        .bind(&art_type)
        .fetch_all(&**pool)
        .await
    {
        Ok(artworks) => HttpResponse::Ok().json(artworks),
        Err(e) => {
            eprintln!("Failed to fetch artworks of type {}: {}", art_type, e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}
