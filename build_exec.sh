#!/bin/bash

cargo build --release
cp target/release/signedurl darwin
rm -rf target

docker build -t buildercontainer:debian -f docker/Dockerfile.debian .
docker create --name=buildercontainer-debian buildercontainer:debian
docker cp buildercontainer-debian:/opt/signedurl/target/release/signedurl debian
docker rm buildercontainer-debian
