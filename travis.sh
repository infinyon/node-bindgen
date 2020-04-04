#!/bin/bash

set -e -o pipefail

source ~/.nvm/nvm.sh
nvm install $NODE || exit 1
nvm use $NODE

# Can't set these until here, as nvm would fail
set -ux

cargo build
cargo test
cargo install --path nj-cli
cd examples && make test