use axum::{
    routing::{get, post},
    Router,
};

use crate::{handler::create_partner, handler::get_partner_id, Conn};

pub fn create_router(pool: Conn) -> Router {
    let router = Router::new()
        .route("/partner/:id", get(get_partner_id))
        .route("/partner/", post(create_partner))
        .with_state(pool);

    router
}
