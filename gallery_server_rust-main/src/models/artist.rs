use serde::{Deserialize, Serialize};
use sqlx::FromRow; // חובה: ודא ששורה זו קיימת!

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)] // חובה: ודא ש-FromRow כאן!
pub struct Artist {
    pub artist_id: Option<String>, // שינוי ל-Option<String>
    pub first_name: String,
    pub last_name: String,
    pub birth_year: i32,
}