#!/usr/bin/env bash

docker build \
  --push \
  --tag ghcr.io/sileader/diffusion-stash/backend:latest \
  --tag ghcr.io/sileader/diffusion-stash/backend:v0.1.1 \
  .
