use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    models::{Geometry, GeometryPoint, Location, Pdv},
    Conn,
};
pub async fn get_partner_id(
    State(ctx): State<Conn>,
    Path(partner_id): Path<i32>,
) -> Response<String> {
    let partner: Result<_, sqlx::Error> = sqlx::query!(
        r#"
        SELECT 
            id::TEXT as "id",
            trading_name as "trading_name",
            owner_name as "owner_name",
            document,
            ST_AsGeoJSON(coverage_area) as "coverage_area",
            ST_AsGeoJSON(address) as "address"
        FROM db.partners WHERE id = $1
        "#,
        partner_id
    )
    .fetch_one(&*ctx)
    .await;

    match partner {
        Ok(record) => {
            let id = record.id.unwrap();
            let trading_name = record.trading_name;
            let owner_name = record.owner_name;
            let document = record.document;
            let coverage_area: Geometry =
                serde_json::from_str(&record.coverage_area.expect("Error getting coverage area"))
                    .expect("Error parsing coverage area");
            let address: GeometryPoint =
                serde_json::from_str(&record.address.expect("Error getting address area"))
                    .expect("Error parsing address area");
            let partner = Pdv {
                id,
                trading_name,
                owner_name,
                document,
                coverage_area,
                address,
            };
            let serialized =
                serde_json::to_string(&partner).expect("Serializing partner went wrong");
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(serialized)
                .unwrap()
        }
        Err(sqlx::Error::RowNotFound) => {
            let mut map = HashMap::new();
            map.insert("message", format!("ID {} has not been found", partner_id));

            let json_string = serde_json::to_string(&map).unwrap();

            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(json_string)
                .unwrap()
        }
        Err(_) => {
            let mut map = HashMap::new();
            map.insert("message", format!("Internal server error"));

            let json_string = serde_json::to_string(&map).unwrap();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(json_string)
                .unwrap()
        }
    }
}

pub async fn create_partner(State(ctx): State<Conn>, Json(pdv): Json<Pdv>) -> impl IntoResponse {
    let result = sqlx::query!(
        r#"
            INSERT INTO db.partners (id, trading_name, owner_name, document, coverage_area, address)
            VALUES ($1, $2, $3, $4, ST_GeomFromGeoJSON($5), ST_GeomFromGeoJSON($6))
            "#,
        pdv.id.parse::<i32>().unwrap(),
        pdv.trading_name,
        pdv.owner_name,
        pdv.document,
        serde_json::to_string(&pdv.coverage_area).unwrap(),
        serde_json::to_string(&pdv.address).unwrap()
    )
    .execute(&*ctx)
    .await;

    match result {
        Ok(_) => {
            let mut map = HashMap::new();
            map.insert("message", format!("Insert has been successful"));

            let json_string = serde_json::to_string(&map).unwrap();

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(json_string)
                .unwrap()
        }
        Err(sqlx::Error::Database(err)) => {
            println!("{:?}", err.message());
            if err.message().contains("partners_pkey") {
                let mut map = HashMap::new();
                map.insert("message", format!("ID already exists"));

                let json_string = serde_json::to_string(&map).unwrap();
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body(json_string)
                    .unwrap()
            } else if err.message().contains("document") {
                let mut map = HashMap::new();
                map.insert("message", format!("Document must be unique"));

                let json_string = serde_json::to_string(&map).unwrap();
                Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .header("Content-Type", "application/json")
                    .body(json_string)
                    .unwrap()
            } else {
                let mut map = HashMap::new();
                map.insert("message", format!("Internal Server Error"));

                let json_string = serde_json::to_string(&map).unwrap();
                Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .header("Content-Type", "application/json")
                    .body(json_string)
                    .unwrap()
            }
        }
        Err(_) => {
            let message = format!("Internal server error");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(message)
                .unwrap()
        }
    }
}

pub async fn find_nearest_partner(
    State(ctx): State<Conn>,
    Path(location): Path<Location>,
) -> Response<String> {
    let partner = sqlx::query!(
        r#"
        SELECT 
            id, 
            trading_name, 
            owner_name,
            document,
            ST_AsGeoJSON(coverage_area) as coverage_area,
            ST_AsGeoJSON(address) as address
        FROM 
            db.partners 
        WHERE 
            ST_Contains(coverage_area, ST_SetSRID(ST_Point($1, $2), 4326))
            ORDER BY 
            ST_Distance(address,ST_SetSRID(ST_Point($1, $2), 4326)) ASC
        LIMIT 1;
        "#,
        location.long,
        location.lat,
    )
    .fetch_one(&*ctx)
    .await;
    println!("{partner:?}");

    match partner {
        Ok(record) => {
            let id = record.id.to_string();
            let trading_name = record.trading_name;
            let owner_name = record.owner_name;
            let document = record.document;
            let coverage_area: Geometry =
                serde_json::from_str(&record.coverage_area.expect("Error getting coverage area"))
                    .expect("Error parsing coverage area");
            let address: GeometryPoint =
                serde_json::from_str(&record.address.expect("Error getting address area"))
                    .expect("Error parsing address area");
            let partner = Pdv {
                id,
                trading_name,
                owner_name,
                document,
                coverage_area,
                address,
            };
            let serialized =
                serde_json::to_string(&partner).expect("Serializing partner went wrong");
            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(serialized)
                .unwrap()
        }
        Err(sqlx::Error::RowNotFound) => {
            let mut map = HashMap::new();
            map.insert("message", format!("Not inside coverage area"));

            let json_string = serde_json::to_string(&map).unwrap();

            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "application/json")
                .body(json_string)
                .unwrap()
        }
        Err(_) => {
            let message = format!("Internal server error");
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("Content-Type", "application/json")
                .body(message)
                .unwrap()
        }
    }
}
