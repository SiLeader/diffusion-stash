#!/usr/bin/env bash

set -e

migration_postgres='migration_postgres'
password='migration'

docker run --rm --name $migration_postgres -e POSTGRES_PASSWORD=$password -e POSTGRES_DB=database -p 5432:5432 -d postgres
sleep 5

url="postgres://postgres:$password@localhost:5432/database"

sea-orm-cli migrate up -d metadata-database/migration -u $url
sea-orm-cli generate entity -u $url -o metadata-database/src/entity --with-serde both

docker stop $migration_postgres
