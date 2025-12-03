use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::SqlitePool; // ודא ש-Row הוסר, כי נשתמש ב-query_as
use crate::models::customer::{Customer};
use uuid::Uuid;

pub async fn init_customers_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    println!("📋 Creating customer table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS customers (
            customer_id TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            email TEXT NOT NULL,
            phone TEXT NOT NULL,
            address TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("✅ customer table ready");
    
    Ok(())
}

#[get("/")] // נתיב יחסי ל-scope של הלקוחות (ככל הנראה "/customers")
pub async fn get_customers(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Customer>(
        r#"
        SELECT customer_id, first_name, last_name, email, phone, address
        FROM customers
        ORDER BY last_name, first_name
        "#
    )
    .fetch_all(&**pool)
    .await
    {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[get("/address/jerusalem")]
pub async fn get_customers_in_jerusalem(pool: web::Data<SqlitePool>) -> impl Responder {
    let search_pattern = "%ירושלים%".to_string();
    match sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE address LIKE ?")
        .bind(search_pattern)
        .fetch_all(&**pool)
        .await
    {
        Ok(customers) => HttpResponse::Ok().json(customers),
        Err(e) => {
            eprintln!("Failed to fetch customers in Jerusalem: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[post("/")]
pub async fn create_customer(pool: web::Data<SqlitePool>, customer: web::Json<Customer>) -> impl Responder {

    let id =Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO CUSTOMERS (customer_id, first_name, last_name, email, phone, address) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&id)
        .bind(&customer.first_name)
        .bind(&customer.last_name)
        .bind(&customer.email)
        .bind(&customer.phone)
        .bind(&customer.address)
        .execute(&**pool)
        .await
    {
        Ok(_result) => {
            let new_customer = Customer {
               customer_id: id,
               first_name: customer.first_name.clone(),
               last_name: customer.last_name.clone(),
               email: customer.email.clone(),
               phone: customer.phone.clone(),
               address: customer.address.clone(),
            };
            HttpResponse::Created().json(new_customer)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
#[delete("/{customer_id}")]
pub async fn delete_customer(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let customer_id = path.into_inner();
    
    match sqlx::query("DELETE FROM customers WHERE customer_id = ?")
        .bind(&customer_id)
        .execute(&**pool)
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                HttpResponse::Ok().body("customer deleted successfully")
            } else {
                HttpResponse::NotFound().body("customer not found")
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}

#[put("/{customer_id}")]
pub async fn update_customer(
    pool: web::Data<SqlitePool>,
    path: web::Path<String>, // Change to String
    updated: web::Json<Customer>,
) -> impl Responder {
    let customer_id = path.into_inner();
    
    // Check if customer exists
    match sqlx::query("SELECT customer_id FROM customers WHERE customer_id = ?") // Corrected query
        .bind(&customer_id)
        .fetch_optional(&**pool)
        .await
    {
        Ok(Some(_)) => {
            match sqlx::query("UPDATE customers SET first_name = ?, last_name = ?, email = ?, phone = ?, address = ? WHERE customer_id = ?") // Corrected table name
                .bind(&updated.first_name)
                .bind(&updated.last_name)
                .bind(&updated.email)
                .bind(&updated.phone)
                .bind(&updated.address)
                .bind(&customer_id) // Bind the customer_id from the path
                .execute(&**pool)
                .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        let updated_customer = Customer {
                            customer_id: customer_id.clone(), // Use the customer_id from the path
                            first_name: updated.first_name.clone(),
                            last_name: updated.last_name.clone(),
                            email: updated.email.clone(),
                            phone: updated.phone.clone(),
                            address: updated.address.clone(),
                        };
                        HttpResponse::Ok().json(updated_customer)
                    } else {
                        HttpResponse::NotFound().body("Customer not found")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Ok(None) => HttpResponse::NotFound().body("Customer with provided customer_id does not exist"), // Corrected response
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}