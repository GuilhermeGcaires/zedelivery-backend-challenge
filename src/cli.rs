use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
struct Partners {
    pdvs: Vec<Pdv>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Pdv {
    id: String,
    #[serde(rename = "tradingName")]
    trading_name: String,
    #[serde(rename = "ownerName")]
    owner_name: String,
    document: String,
    #[serde(rename = "coverageArea")]
    coverage_area: Geometry,
    address: GeometryPoint,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geometry {
    #[serde(rename = "type")]
    geo_type: String,
    coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeometryPoint {
    #[serde(rename = "type")]
    geo_type: String,
    coordinates: Vec<f64>,
}

pub async fn insert_partners_from_file(pool: &PgPool) -> Result<(), Error> {
    let json_data = fs::read_to_string("partners.json").expect("Unable to read file");

    let parsed_data: Partners = serde_json::from_str(&json_data).expect("Error parsing the JSON");

    for pdv in parsed_data.pdvs {
        // sqlx::query!(
        //     r#"
        //     INSERT INTO db.partners (id, trading_name, owner_name, document, coverage_area, address)
        //     VALUES ($1, $2, $3, $4, $5, $6)
        //     "#,
        //     partner.id,
        //     partner.tradingName,
        //     partner.ownerName,
        //     partner.document,
        //     serde_json::to_string(&partner.coverageArea).unwrap(),
        //     serde_json::to_string(&partner.address).unwrap()
        // )
        // .execute(pool)
        // .await?;
        println!("{:?}", &pdv.coverage_area)
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Call the insert function
    if let Err(error) = insert_partners_from_file(&pool).await {
        eprintln!("Error inserting partners: {}", error);
    }
}
