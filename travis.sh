#!/bin/bash

set -eux -o pipefail

source ~/.nvm/nvm.sh
nvm install $NODE
nvm use $NODE
cargo build
cargo test
cargo install --path nj-cli
cd examples && make test