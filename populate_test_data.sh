#!/usr/bin/env bash

export PGPASSWORD="mysecretpassword"
export PGHOST="localhost"
export PGUSER=postgres

dropdb file
dropdb user
dropdb article
createdb file
createdb article
createdb user
psql -f src/test_data/article.sql  article
psql -f src/test_data/user.sql  user