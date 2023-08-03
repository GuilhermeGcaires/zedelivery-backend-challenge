-- Add migration script here
CREATE EXTENSION postgis;
CREATE SCHEMA db;
CREATE TABLE db.partners (
    id SERIAL PRIMARY KEY,
    trading_name VARCHAR(100) NOT NULL,
    owner_name VARCHAR(100) NOT NULL,
    document VARCHAR(100) UNIQUE NOT NULL,
    coverage_area GEOMETRY NOT NULL,
    address GEOMETRY NOT NULL
);
