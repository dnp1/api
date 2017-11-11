#!/bin/bash

export PGPASSWORD="mysecretpassword"
export PGHOST="localhost"
export PGUSER=postgres

dropdb file
dropdb user
dropdb article
createdb file
createdb article
createdb user
psql -f src/ddl/article.sql  article
psql -f src/ddl/user.sql  user
psql -f src/ddl/file.sql  file