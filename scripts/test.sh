#!/bin/bash
set -eux -o pipefail

main() {
  find . | grep -v /target | grep -v "/\." | entr -s \
    'date \
    && pushd ch58x-ble-rt && cargo build --examples && popd \
    && pushd linux_client && cargo test --examples && popd'
}

main "$@"
