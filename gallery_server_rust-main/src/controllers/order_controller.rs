use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::order::{Order};
use uuid::Uuid;
use serde_json::json;

pub async fn init_orders_table(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ORDERS (
            id_order TEXT PRIMARY KEY NOT NULL,
            id_customer TEXT NOT NULL,
            order_date DATE NOT NULL,
            FOREIGN KEY (id_customer) REFERENCES customers(customer_id)
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[get("/")]
pub async fn get_orders(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query_as::<_, Order>("SELECT id_order, id_customer, order_date FROM ORDERS")
        .fetch_all(&**pool)
        .await
    {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[post("/")]
pub async fn create_order(pool: web::Data<SqlitePool>, order: web::Json<Order>) -> impl Responder {
    let id =Uuid::new_v4().to_string();
    match sqlx::query("INSERT INTO ORDERS (id_order, id_customer, order_date) VALUES (?, ?, ?) RETURNING id_order, id_customer, order_date")
        .bind(&id)
        .bind(&order.id_customer)
        .bind(&order.order_date)
        .execute(&**pool)
        .await
    {
        Ok(_result) => {
            let new_order = Order {
                id_order: id,
                id_customer: order.id_customer.clone(),
                order_date: order.order_date.clone(),
            };
            HttpResponse::Created().json(new_order)         
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
    }
}
#[get("/{id_order}")]
pub async fn get_order_by_id(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let id_order = path.into_inner();
    match sqlx::query_as::<_, Order>("SELECT id_order, id_customer, order_date FROM ORDERS WHERE id_order = ?")
        .bind(&id_order)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[put("/{id_order}")]
pub async fn update_order(pool: web::Data<SqlitePool>, path: web::Path<String>, updated_order: web::Json<Order>) -> impl Responder {
    let id_order = path.into_inner();
    match sqlx::query(
        "UPDATE ORDERS SET id_customer = ?, order_date = ? WHERE id_order = ?"
    )
        .bind(&updated_order.id_customer)
        .bind(&updated_order.order_date)
        .bind(&id_order)
        .execute(pool.get_ref())
        .await
    {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let updated_order_response = Order {
                    id_order: id_order.clone(),
                    id_customer: updated_order.id_customer.clone(),
                    order_date: updated_order.order_date.clone(),
                };
                HttpResponse::Ok().json(updated_order_response)
            } else {
                HttpResponse::NotFound().body(format!("Order with id {} not found", id_order))
            }
        }
        Err(e) => {
            eprintln!("Failed to update order with id {}: {}", id_order, e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[delete("/{id_order}")]
pub async fn delete_order(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let id_order = path.into_inner();

    // Delete related artworks in order first
    match sqlx::query("DELETE FROM artworks_in_order WHERE id_order = ?")
        .bind(&id_order)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => {
            // Now delete the order
            match sqlx::query("DELETE FROM ORDERS WHERE id_order = ?")
                .bind(&id_order)
                .execute(pool.get_ref())
                .await
            {
                Ok(result) => {
                    if result.rows_affected() > 0 {
                        HttpResponse::Ok().body("order deleted successfully")
                    } else {
                        HttpResponse::NotFound().body("order not found")
                    }
                }
                Err(e) => HttpResponse::InternalServerError().body(format!("Database error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Database error while deleting artworks in order: {}", e)),
    }
}

#[get("/after/2025-01-01")]
pub async fn get_orders_after_2025_01_01(pool: web::Data<SqlitePool>) -> impl Responder {
    let date_str = "2025-01-01".to_string();
    match sqlx::query_as::<_, Order>("SELECT id_order, id_customer, order_date FROM ORDERS WHERE order_date > ?")
        .bind(date_str)
        .fetch_all(&**pool)
        .await
    {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Failed to fetch orders after 2025-01-01: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[get("/detailed")]
pub async fn get_detailed_orders(pool: web::Data<SqlitePool>) -> impl Responder {
    let query = r#"
        SELECT
            o.id_order,
            o.order_date,
            c.customer_id,
            c.first_name,
            c.last_name,
            c.email,
            c.phone,
            c.address,
            aio.id_artwork_in_order,
            aio.id_artwork,
            aio.amount,
            a.title AS artwork_title,
            a.description AS artwork_description,
            a.year_created AS artwork_year_created,
            a.price AS artwork_price,
            a.id_artist AS artwork_id_artist,
            a.art_type AS artwork_art_type
        FROM ORDERS o
        JOIN CUSTOMERS c ON o.id_customer = c.customer_id
        LEFT JOIN artworks_in_order aio ON o.id_order = aio.id_order
        LEFT JOIN ARTWORKS a ON aio.id_artwork = a.id_artwork
        ORDER BY o.id_order, aio.id_artwork_in_order
    "#;

    match sqlx::query(query)
        .fetch_all(&**pool)
        .await
    {
        Ok(rows) => {
            let mut orders_map: std::collections::HashMap<String, serde_json::Value> = std::collections::HashMap::new();

            for row in rows {
                let order_id: String = row.get("id_order");
                
                let order_entry = orders_map.entry(order_id.clone()).or_insert_with(|| {
                    json!({
                        "id_order": order_id,
                        "order_date": row.get::<String, _>("order_date"),
                        "customer": {
                            "customer_id": row.get::<String, _>("customer_id"),
                            "first_name": row.get::<String, _>("first_name"),
                            "last_name": row.get::<String, _>("last_name"),
                            "email": row.get::<String, _>("email"),
                            "phone": row.get::<String, _>("phone"),
                            "address": row.get::<String, _>("address"),
                        },
                        "artworks": [],
                        "total_amount": 0.0f64,
                    })
                });

                if let Some(artwork_in_order_id) = row.try_get::<String, _>("id_artwork_in_order").ok() {
                    let artwork_amount: i32 = row.get("amount");
                    let artwork_price: f64 = row.get("artwork_price");

                    let artwork_total = artwork_amount as f64 * artwork_price;

                    if let Some(artworks_array) = order_entry["artworks"].as_array_mut() {
                        artworks_array.push(json!({
                            "id_artwork_in_order": artwork_in_order_id,
                            "id_artwork": row.get::<String, _>("id_artwork"),
                            "amount": artwork_amount,
                            "artwork_title": row.get::<String, _>("artwork_title"),
                            "artwork_description": row.get::<String, _>("artwork_description"),
                            "artwork_year_created": row.get::<i32, _>("artwork_year_created"),
                            "artwork_price": artwork_price,
                            "artwork_id_artist": row.get::<String, _>("artwork_id_artist"),
                            "artwork_art_type": row.get::<String, _>("artwork_art_type"),
                            "total_price_for_artwork": artwork_total,
                        }));
                    }
                    
                    let current_total = order_entry["total_amount"].as_f64().unwrap_or(0.0);
                    order_entry["total_amount"] = json!(current_total + artwork_total);
                }
            }
            HttpResponse::Ok().json(orders_map.values().collect::<Vec<&serde_json::Value>>())  
        }
        Err(e) => {
            eprintln!("Failed to fetch detailed orders: {}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}
