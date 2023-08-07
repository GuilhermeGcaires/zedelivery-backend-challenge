use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    handler::get_partner_id,
    handler::{create_partner, find_nearest_partner},
    Conn,
};

pub fn create_router(pool: Conn) -> Router {
    let router = Router::new()
        .route("/partner/:id", get(get_partner_id))
        .route("/partner/", post(create_partner))
        .route("/find_partner/:long/:lat", get(find_nearest_partner))
        .with_state(pool);

    router
}
