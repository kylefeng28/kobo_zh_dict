#!/bin/bash

set -euo pipefail

CEDICT_FILE='cedict_1_0_ts_utf-8_mdbg.txt'
CEDICT_FILE_GZ="${CEDICT_FILE}.gz"

if [[ -f "$CEDICT_FILE" ]]; then
  echo "CC-CEDICT file exists at $CEDICT_FILE"
else
  echo "Downloading CC-CEDICT file..."
  curl -LO "https://www.mdbg.net/chinese/export/cedict/$CEDICT_FILE_GZ"
  gunzip "$CEDICT_FILE_GZ"
fi

gen() {
  cargo run -- -c "$1" -p "$2" --output "dicthtml-zh-en-$1-$2.zip"
}

gen simplified pinyin
gen traditional pinyin
gen traditional zhuyin
