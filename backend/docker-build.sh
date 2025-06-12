#!/usr/bin/env bash

version=$(cat Cargo.toml | grep version | awk '{print $3}' | tr -d '"')

docker build \
  --push \
  --tag ghcr.io/sileader/diffusion-stash/backend:latest \
  --tag ghcr.io/sileader/diffusion-stash/backend:v$version \
  .
