use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool; // ודא שאין כאן Row
use crate::models::artist::Artist;
use uuid::Uuid;

pub async fn init_artists_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("📋 Creating artists table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS artists (
            artist_id TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            birth_year INTEGER NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    println!("✅ artists table ready");
    
    Ok(())
}

#[get("/")] // הנתיב הריק יתייחס לנתיב הבסיסי של ה-scope, כלומר "/artists"
pub async fn get_artists(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Artist>(
        r#"
        SELECT artist_id, first_name, last_name, birth_year
        FROM artists
        ORDER BY last_name, first_name
        "#,
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(artists) => HttpResponse::Ok().json(artists),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[post("/")] // הנתיב הריק יתייחס לנתיב הבסיסי של ה-scope, כלומר "/artists"
pub async fn create_artist(pool: web::Data<SqlitePool>, artist: web::Json<Artist>) -> impl Responder {
    let id = Uuid::new_v4().to_string();
    match sqlx::query(
        r#"
        INSERT INTO artists (artist_id, first_name, last_name, birth_year)
        VALUES (?, ?, ?, ?)
        "#
    )
    .bind(&id)
    .bind(&artist.first_name)
    .bind(&artist.last_name)
    .bind(&artist.birth_year)
    .execute(&**pool)
    .await
    {
        Ok(_) => {
            let new_artist = Artist {
                artist_id: Some(id),
                first_name: artist.first_name.clone(),
                last_name: artist.last_name.clone(),
                birth_year: artist.birth_year,
            };
            HttpResponse::Created().json(new_artist)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[delete("/{artist_id}")] // ודאי שזה "/{artist_id}"
pub async fn delete_artist(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let artist_id = path.into_inner();
    
    match sqlx::query("DELETE FROM artists WHERE artist_id = ?")
        .bind(&artist_id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("Artist deleted successfully")
            } else {
                HttpResponse::NotFound().body("Artist not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{artist_id}")] // ודאי שזה "/{artist_id}"
pub async fn update_artist(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>,
    updated: web::Json<Artist>,
) -> impl Responder {
    let artist_id = path.into_inner();

    match sqlx::query("SELECT artist_id FROM artists WHERE artist_id = ?")
        .bind(&artist_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query(
                r#"
                UPDATE artists SET first_name = ?, last_name = ?, birth_year = ?
                WHERE artist_id = ?
                "#
            )
            .bind(&updated.first_name)
            .bind(&updated.last_name)
            .bind(&updated.birth_year)
            .bind(&artist_id)
            .execute(&**pool)
            .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        let updated_artist = Artist {
                            artist_id: Some(artist_id.clone()),
                            first_name: updated.first_name.clone(),
                            last_name: updated.last_name.clone(),
                            birth_year: updated.birth_year,
                        };
                        HttpResponse::Ok().json(updated_artist)
                    } else {
                        HttpResponse::NotFound().body("Artist not found")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Artist with provided artist_id does not exist"),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/born_after_1980")]
pub async fn get_artists_born_after_1980(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Artist>("SELECT * FROM artists WHERE birth_year > 1980")
        .fetch_all(&**pool)
        .await
    {
        Ok(artists) => HttpResponse::Ok().json(artists),
        Err(e) => {
            eprintln!("Failed to fetch artists born after 1980: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}