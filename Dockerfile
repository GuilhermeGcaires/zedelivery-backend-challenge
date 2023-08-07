FROM rust:1.70 as builder

WORKDIR /usr/src/ze_partner_service
COPY . .

RUN cargo install sqlx-cli

RUN sqlx migrate run --database-url $DATABASE_URL

# RUN cargo run
#
# FROM debian:buster-slim
#
# RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*
#
# COPY --from=builder /usr/local/cargo/bin/ze_partner_service /usr/local/bin/ze_partner_service
#
# CMD ["ze_partner_service"]

