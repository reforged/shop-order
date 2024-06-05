use std::sync::Arc;
use sqlx::PgPool;
use tracing::info;
use super::model::CreateOrder;
use sqlx::Row;
use sqlx::types::Json;
use uuid::Uuid;

pub struct OrderService {
    pool: Arc<PgPool>
}

impl OrderService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn create(&self, order: CreateOrder) -> Result<Uuid, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        println!("Order: {:?}", order);

        // log the order
        let order_json = Json(order.clone());
        info!("Order: {:?}", order_json);

        let result = sqlx::query("INSERT INTO orders (customer_id, total_amount, status) VALUES ($1, 0, 'pending') RETURNING id")
            .bind(order.customer_id)
            .fetch_one(transaction.as_mut())
            .await;

        let order_id: Uuid = match result {
            Ok(row) => row.get(0),
            Err(e) => {
                println!("Error: {:?}", e);
                return Err(e);
            }
        };

        for item in order.items {
            println!("Item: {:?}", item);
            // product_id : UUID
            let product_id = Uuid::parse_str(&item.product_id).unwrap();
            let result = sqlx::query("INSERT INTO order_items (order_id, product_id, quantity, price) VALUES ($1, $2, $3, $4)")
                .bind(order_id)
                .bind(product_id)
                .bind(item.quantity)
                .bind(item.price)
                .execute(transaction.as_mut())
                .await;

            match result {
                Ok(_) => (),
                Err(e) => {
                    println!("Error: {:?}", e);
                    transaction.rollback().await?;
                    return Err(e);
                }
            }
        }

        transaction.commit().await?;

        Ok(order_id)
    }
}