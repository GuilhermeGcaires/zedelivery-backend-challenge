mod handler;
mod route;

use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use dotenv::dotenv;
use route::create_router;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type Conn = Arc<Pool<Postgres>>;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_connection_str = std::env::var("DATABASE_URL").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("could not connect to database url");

    println!("{:?}", pool);

    let state_pool = Arc::new(pool);

    let app = create_router(state_pool);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
