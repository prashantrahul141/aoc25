#!/usr/bin/env bash

SOURCE_DIR=$(realpath "$(dirname "$0")")
INPUT_DIR=${INPUT_DIR:="$SOURCE_DIR/src"}

help () {
  echo "usage:   $0 <day1-day12>"
  echo "example: $0 day05"
  echo "         $0 05"
  echo "         $0 all"
}

fetch () {
  TMPFILE=$(mktemp)
  curl "https://adventofcode.com/2025/day/${1}/input"              \
    -s --fail-with-body --cookie "session=$AOC_SESSION"            \
    -o "${TMPFILE}"

  day_folder=$(printf "_%02d" "${1}")
  mkdir -p "$INPUT_DIR/${day_folder}"
  mv "$TMPFILE" "$INPUT_DIR/${day_folder}/input"
  echo "Fetched Day $1"
}

if [[ $# != 1 || ! "${1#day}" =~ ^([1-9]|1[0-9]|2[0-5]|all)$ ]]; then
  help
  exit 1
fi

if [[ -z "${AOC_SESSION}" ]]; then
  echo "\$AOC_SESSION is not set"
  exit 1
fi

if [[ $1 == "all" ]]; then
    for x in {1..12}; do fetch "$x"; done
else
    fetch "${1#day}"
fi
