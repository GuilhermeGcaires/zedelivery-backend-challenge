use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{handler::get_partner_id, Conn};

pub fn create_router(pool: Conn) -> Router {
    let router = Router::new()
        .route("/partner/:id", get(get_partner_id))
        .with_state(pool);

    router
}
