use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
pub struct Artwork {
    pub id_artwork: Option<String>,
    pub title: String,
    pub description: String,
    pub year_created: i32,
    pub price: f64,
    pub id_artist: String,
    pub art_type: String,
}
