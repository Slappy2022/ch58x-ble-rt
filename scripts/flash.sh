#!/bin/bash
set -eux -o pipefail

readonly BASE_DIR="$(
  cd -P "$(dirname "${BASH_SOURCE[0]}")/.."
  pwd
)"

main() {
  local -r app=$1
  cd "${BASE_DIR}/ch58x-ble-rt"
  cargo run --example "${app}"
  while :; do
    sleep 0.5
    "${BASE_DIR}"/scripts/client.sh "${app}"
  done
}

main "$@"
