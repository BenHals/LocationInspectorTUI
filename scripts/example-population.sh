#!/bin/bash
# Example layer: hardcoded population per region.
#
# Demonstrates the simplest layer: a static lookup table. No network,
# no disk I/O — just maps requested region IDs to numeric values.
#
# Population figures are approximate and sourced from public references
# (Wikipedia, ~2023). Boundary polygons are intentionally omitted from
# the table to show the "missing value" rendering path.
set -euo pipefail

table='{
  "nyc-staten-island": 495747,
  "nyc-queens":        2405464,
  "nyc-brooklyn":      2736074,
  "nyc-manhattan":     1694251,
  "nyc-bronx":         1472654,

  "lon-camden":              270029,
  "lon-greenwich":           287942,
  "lon-hammersmith-fulham":  183200,
  "lon-newham":              351000,
  "lon-westminster":         261000,

  "tyo-chiyoda":   67485,
  "tyo-shinagawa": 415000,
  "tyo-shinjuku":  348000,
  "tyo-shibuya":   230000,
  "tyo-minato":    260000,

  "syd-sydney":       250000,
  "syd-north-sydney": 73000,
  "syd-bondi":        11656,
  "syd-surry-hills":  4500,
  "syd-balmain":      10500
}'

# Filter the table down to just the requested IDs (preserve those that exist).
jq -c --argjson table "$table" '
  reduce .[] as $id ({};
    if $table | has($id) then .[$id] = $table[$id] else . end)
'
