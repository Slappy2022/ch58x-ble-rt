#!/bin/bash
set -eux

readonly BASE_DIR="$(
  cd -P "$(dirname "${BASH_SOURCE[0]}")/.."
  pwd
)"

main() {
  local -r app=$1
  cd "${BASE_DIR}/"linux_client
  cargo run --example "${app}"
}

main "$@"
