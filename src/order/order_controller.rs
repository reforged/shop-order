use std::sync::Arc;

use actix_web::{post, web, HttpResponse, Responder};
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};


use crate::order::{model::{CreateOrder, OrderVerifStockMessage}, order_service::OrderService};



#[post("/orders")]
pub async fn create_order(
    order: web::Json<CreateOrder>,
    order_service: web::Data<Arc<OrderService>>,
    channel: web::Data<Arc<Channel>>
) -> impl Responder {
    format!("Order: {:?}", order.clone());

    let order_id = order_service.create(order.clone()).await;

    match order_id {
        Ok(order_id) => {
            let order_verif_stock_message = OrderVerifStockMessage {
                order_id: order_id.to_string(),
                items: order.items.clone()
            };

            let json_string = serde_json::to_string(&order_verif_stock_message).unwrap();
            let content = json_string.as_bytes();
           
            channel.basic_publish(
                "",
                "test",
                BasicPublishOptions::default(),
                content,
                BasicProperties::default()
            ).await.unwrap();
        },
        Err(e) => {
            println!("Error: {:?}", e);
        },
    };

    HttpResponse::Created().json(order)
}