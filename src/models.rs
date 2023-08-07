use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Pdv {
    pub id: String,
    #[serde(rename = "tradingName")]
    pub trading_name: String,
    #[serde(rename = "ownerName")]
    pub owner_name: String,
    pub document: String,
    #[serde(rename = "coverageArea")]
    pub coverage_area: Geometry,
    pub address: GeometryPoint,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    #[serde(rename = "type")]
    geo_type: String,
    coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeometryPoint {
    #[serde(rename = "type")]
    geo_type: String,
    coordinates: Vec<f64>,
}

#[derive(Deserialize)]
pub struct Location {
    pub long: f64,
    pub lat: f64,
}
