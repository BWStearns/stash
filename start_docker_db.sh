#!/usr/bin/env bash

# pull postgres image if needed
docker pull postgres
# start postgres container on port 1515
docker run \
    --name stash \
    -p 5432:5432 \
    -e POSTGRES_USER=stash \
    -e POSTGRES_PASSWORD=stashpass \
    -e POSTGRES_DB=stash \
    -d \
    postgres
