#!/usr/bin/env bash

version=$(cat package.json | jq -r '.version')

docker build \
  --push \
  --tag ghcr.io/sileader/diffusion-stash/frontend:latest \
  --tag ghcr.io/sileader/diffusion-stash/frontend:v$version \
  .
