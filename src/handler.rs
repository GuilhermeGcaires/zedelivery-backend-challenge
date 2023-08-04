use axum::extract::{Path, State};

use crate::Conn;

pub async fn get_partner_id(State(ctx): State<Conn>, Path(partner_id): Path<i32>) {
    println!("{ctx:?}");
    let partner = sqlx::query!(
        r#"
        SELECT row_to_json(partners.*)FROM db.partners WHERE id = $1
        "#,
        partner_id
    )
    .fetch_optional(&*ctx)
    .await;
    println!("{:?}", partner);
}
