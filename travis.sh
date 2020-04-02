#!/bin/bash

source ~/.nvm/nvm.sh

set -eux -o pipefail

nvm install $NODE
nvm use $NODE
cargo build
cargo test
cargo install --path nj-cli
cd examples && make test