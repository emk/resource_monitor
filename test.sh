#!/bin/bash

# Standard paranoia.
set -euo pipefail

# Build a test image and run it with Docker memory limits enabled.
cargo build --example=use_all_memory --target=i686-unknown-linux-musl
docker-compose build
docker-compose run test
