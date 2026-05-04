#!/bin/bash
# Example layer: elevation in meters via the open-elevation.com public API.
#
# Demonstrates a layer that combines on-disk data with a live network call:
# centroids are computed from the polygon ring data, then sent to the API.
# Because centroids come from the data itself, this layer works for any
# region in the dataset with no per-region hardcoding.
#
# Polygon coordinates are in UTM meters recentered around the city's lat/lng
# (looked up from locations.json). A flat-earth approximation converts the
# centroid back to lat/lng — accurate enough for elevation lookup (the API
# uses 30m SRTM grid).
#
# Failure modes (network down, API offline, non-200 response) propagate via
# non-zero exit; the app surfaces this as a Failed layer state.
set -euo pipefail

LOCATIONS_FILE="${LOCTUI_DATA_ROOT}/locations.json"
POLY_FILE="${LOCTUI_DATA_ROOT}/polygons/${LOCTUI_LOCATION_ID}.json"

ids_json="$(cat)"

city_lng="$(jq -r --arg id "$LOCTUI_LOCATION_ID" '.[] | select(.id == $id) | .coord[0]' "$LOCATIONS_FILE")"
city_lat="$(jq -r --arg id "$LOCTUI_LOCATION_ID" '.[] | select(.id == $id) | .coord[1]' "$LOCATIONS_FILE")"

# Compute centroid per region, then convert to WGS84.
filtered="$(jq -c \
  --argjson ids "$ids_json" \
  --argjson city_lat "$city_lat" \
  --argjson city_lng "$city_lng" '
  def centroid:
    .[:-1] as $pts
    | ($pts | length) as $n
    | [(($pts | map(.[0]) | add) / $n), (($pts | map(.[1]) | add) / $n)];
  (.regions | map({key: .id, value: (.ring | centroid)}) | from_entries) as $c
  | ($city_lat * 0.017453292519943295) as $lat_rad
  | (111320.0 * ($lat_rad | cos)) as $m_per_deg_lng
  | $ids
    | map(. as $id | select($c | has($id)))
    | map(. as $id | $c[$id] as $cm | {
        id: $id,
        lat: ($city_lat + $cm[1] / 111320.0),
        lng: ($city_lng + $cm[0] / $m_per_deg_lng)
      })
' "$POLY_FILE")"

if [ "$(jq 'length' <<<"$filtered")" -eq 0 ]; then
  echo '{}'
  exit 0
fi

payload="$(jq -c '{locations: map({latitude: .lat, longitude: .lng})}' <<<"$filtered")"

response="$(curl -sfS --max-time 15 -X POST "https://api.open-elevation.com/api/v1/lookup" \
  -H "Content-Type: application/json" \
  -d "$payload")"

jq -nc --argjson rows "$filtered" --argjson resp "$response" '
  ($rows | map(.id)) as $ids
  | ($resp.results | map(.elevation)) as $elevs
  | reduce range(0; $ids | length) as $i ({}; .[$ids[$i]] = $elevs[$i])
'
