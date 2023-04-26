#!/usr/bin/env bash

set -x
set -eo pipefail

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
    #  Increased max number of connections for testing purposes
