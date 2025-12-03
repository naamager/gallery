use serde::{Deserialize, Serialize};
use sqlx::FromRow; // חובה: ודא ששורה זו קיימת!

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)] // חובה: ודא ש-FromRow כאן!
pub struct Customer {
    pub customer_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}