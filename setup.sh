#!/usr/bin/env bash
set -ex

# Install dev dependencies
cargo install httpmock --features standalone
