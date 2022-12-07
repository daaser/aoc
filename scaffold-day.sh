#!/usr/bin/env bash

if [[ -z ${SESSION} ]]; then
  echo "SESSION must be set"
  exit 1
fi

DAY=$(date -u +"%-d")
PREVIOUS_DAY=$((DAY-1))
BASEPATH="./src/day${DAY}"

if [[ -d "${BASEPATH}" ]] && [[ -f "${BASEPATH}/mod.rs" ]]; then
  echo "Day already scaffolded"
  exit 0
fi

set -x
sed -i "" "s/mod day${PREVIOUS_DAY};/mod day${PREVIOUS_DAY};\nmod day${DAY};/" ./src/main.rs
sed -i "" "s/day!(${PREVIOUS_DAY});/day!(${PREVIOUS_DAY});\n  day!(${DAY});/" ./src/main.rs

mkdir "${BASEPATH}"
cp mod.rs.tmpl "${BASEPATH}/mod.rs"

curl -so "${BASEPATH}/input.txt" \
  --cookie session="${SESSION}" \
  "https://adventofcode.com/2022/day/${DAY}/input"