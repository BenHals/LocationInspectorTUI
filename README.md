# Location Inspector
_Mainly a learning exercise for Rust!_

<WORK IN PROGRESS>

This is a TUI application written in rust which displays information about geographic location in the terminal.

For example, we may have a backend with a list of cities each with:
- A location in the world
- A set of points or areas of interest in local space (suburbs, train stations etc.)
- Some timeseries information (weather, population, etc.)
- Static metadata (Name, Country, etc)

The aim to to provide a nice visual way to quickly look up this data for a given location, as well as compare locations.

## Configuration

Point the app at a TOML config via the `LOCTUI_CONFIG` env var:

```bash
LOCTUI_CONFIG=config/loctui.toml cargo run
```

If unset, the app falls back to a built-in default (uses `app/data/example_data` and no layers).

### Environment variables

| Variable | Direction | Purpose |
|---|---|---|
| `LOCTUI_CONFIG` | input to app | path to a TOML config file; falls back to built-in defaults if unset |
| `LOCTUI_LOCATION_ID` | passed to layer scripts | id of the currently-inspected location (from `locations.json`) |
| `LOCTUI_DATA_ROOT` | passed to layer scripts | resolved `data.root_dir` from the active config; lets scripts locate polygon files without hardcoded paths |
