use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    id: String,
    customer_id: String,
    pub items: Vec<OrderItem>,
    pub total_amount: f64,
    pub status: OrderStatus,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderItem {
    pub product_id: String,
    pub quantity: i32,
    pub price: f64
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OrderStatus {
    Pending,
    Processing,
    Completed,
    Cancelled
} 


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreateOrder {
    pub items: Vec<OrderItem>,
    pub customer_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderVerifStockMessage {
    pub order_id: String,
    pub items: Vec<OrderItem>
}