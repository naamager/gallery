use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct ArtworkInOrder {
    pub id_artwork_in_order: String,
    pub id_order: String,
    pub id_artwork: String,
    pub amount: i32,
}
