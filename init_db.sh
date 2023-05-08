#!/usr/bin/env bash
set -x
set -eo pipefail

# Check if sqlx installed or not
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    exit 1
fi

# Check if psql is installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi

DB_USER="${POSTGRES_USER:=postgres}"
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${PORTGRES_HOST:=localhost}"

if [[ -z "${SKIP_DOCKER}" ]]
then
docker run \
    -e POSTGRES_USER=${DB_USER} \
    -e POSTGRES_PASSWORD=${DB_PASSWORD} \
    -e POSTGRES_DB=${DB_NAME} \
    -p "${DB_PORT}:${DB_PORT}" \
    -d postgres \
    postgres -N 1000
fi

export PGPASSWORD=${DB_PASSWORD}
until psql -h "${DB_HOST}" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
>&2 echo "Postgress is still unavailable - sleeping"
sleep 2
done

echo "Postgress is up and runnign on port - ${DB_PORT} - running migrations now!"

DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL

sqlx database create
sqlx migrate run

echo "Migrations are applied successfully"
