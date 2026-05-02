#!/bin/bash
# Example layer: polygon area in km², computed from the on-disk polygon data.
#
# Demonstrates a layer that reads from the same dataset the app loads from.
# Uses the shoelace formula on the projected (UTM, meters) ring coordinates,
# then converts to km².
#
# LOCTUI_LOCATION_ID and LOCTUI_DATA_ROOT are provided as env vars by the app.
set -euo pipefail

POLY_FILE="${LOCTUI_DATA_ROOT}/polygons/${LOCTUI_LOCATION_ID}.json"

ids_json="$(cat)"

jq -c --argjson ids "$ids_json" '
  def shoelace:
    . as $r
    | reduce range(0; ($r | length) - 1) as $i (0;
        . + $r[$i][0] * $r[$i+1][1] - $r[$i+1][0] * $r[$i][1])
    | . / 2
    | if . < 0 then -. else . end;

  (map({key: .id, value: ((.ring | shoelace) / 1000000)}) | from_entries) as $all
  | reduce $ids[] as $id ({};
      if $all | has($id) then .[$id] = $all[$id] else . end)
' "$POLY_FILE"
