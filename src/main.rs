mod order;
use std::{env, sync::Arc};

use actix_web::{App, HttpServer, web::Data};
use dotenv::dotenv;

use lapin::Connection;
use order::{order_controller::create_order, order_service::{OrderService}};
use sqlx::postgres::PgPoolOptions;
use tracing::info;


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let conn = Connection::connect("amqp://rabbit:rabbit@localhost/%2f", lapin::ConnectionProperties::default()).await?;

    let channel = conn.create_channel().await?;


    let addr = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("3333".to_string());

    let db_password =
        env::var("DB_PASSWORD").expect("DB_PASSWORD must be set in .env file");
    let db_url = env::var("DB_URL").expect("DB_URL must be set in .env file");
    let db_user = env::var("DB_USER").expect("DB_USER must be set in .env file");

    let database_url = format!(
        "postgres://{}:{}@{}/order_service",
        db_user, db_password, db_url
    );


    let addr_in = format!("{}:{}", addr, port);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    info!("Starting server at: {}", addr_in);


    let pool = Arc::new(pool);
    let channel = Arc::new(channel.clone());
    let order_service = Arc::new(OrderService::new(pool.clone()));
    
    // let payload = b"Hello world!";

    // channel.as_ref()
    //     .basic_publish(
    //         "",
    //         "test",
    //         BasicPublishOptions::default(),
    //         payload,
    //         BasicProperties::default()
    //     ).await?;
    


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(channel.clone()))
            .app_data(Data::new(order_service.clone()))
            .service(create_order)
    })
    .bind(addr_in)?
    .run()
    .await?;

    Ok(())
}