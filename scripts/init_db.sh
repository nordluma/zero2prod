#!/usr/bin/env bash

set -x
set -eo pipefail

if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 "cargo install sqlx"
    echo >&2 "to install it"
    exit
fi

# Check if a custom user has beens set, otherwise default to "postgres"
DB_USER="${POSTGRES_USER:=postgres}"
# Check if a custom password has been set, otherwise default to "postgres"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
# Check if a custom database has been set, otherwise default to "postgres"
DB_NAME="${POSTGRES_DB:=newsletter}"
# Check if a cutom port has been set, otherwise default to "5432"
DB_PORT="${POSTGRES_PORT:=5432}"
# Check if a custom host has been set, otherwise default to "localhost"
DB_HOST="${POSTGRES_HOST:=localhost}"

# Launch postgres using Docker
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}":5432 \
    -d postgres \
    postgres -N 1000
# Increased max number of connections for testing purposes

# Ping postgres until it's ready to accept commands
until PGPASSWORD="${DB_PASSWORD}" psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
  >&2 echo "Postgres is still unavailable - sleeping"
  sleep 1
done

>&2 echo "Postgres is up and running on port ${DB_PORT}!"

DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}
export DATABASE_URL
sqlx migrate add create_subscription_table 
