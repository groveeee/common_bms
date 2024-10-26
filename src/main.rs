use std::{sync::Arc, thread::AccessError};

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, STRICT_TRANSPORT_SECURITY},
    HeaderValue, Method,
};
use config::Config;
use dotenv::dotenv;
use log::{error, info};
use redis::{AsyncCommands, RedisConnectionInfo, RedisResult};
use redis::aio::MultiplexedConnection;
use routes::create_router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::cors::CorsLayer;
use tracing_subscriber::filter::LevelFilter;
use crate::utils::ip_util::get_local_ip;

mod config;
mod routes;
mod error;
mod utils;


#[derive(Clone, Debug)]
pub struct AppState {
    pub env: Config,
    pub pg: Pool<Postgres>,
    pub redis: MultiplexedConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();

    dotenv().ok();

    let config = Config::init();
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to the database is successful!");
            pool
        }
        Err(e) => {
            error!("Failed to connect to the database:{:?}", e);
            std::process::exit(1);
        }
    };
    let client = match redis::Client::open(config.redis_url.clone()) {
        Ok(client) => {
            info!("Successfully connected to the Redis !");
            client
        }
        Err(e) => {
            error!("Failed to connect to the Redis:{:?}", e);
            std::process::exit(1);
        }
    };
    let connection = client.get_multiplexed_tokio_connection().await.unwrap();

    let app_state = AppState {
        env: config.clone(),
        pg: pool,
        redis: connection,
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = create_router(Arc::new(app_state.clone())).layer(cors);

    let addr = get_local_ip();
    info!("Server is running on http://{}:{}", addr,config.port);


    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", &config.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
