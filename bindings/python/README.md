# Wickra Shazam — Python

Python bindings for [wickra-shazam](https://github.com/wickra-lib/wickra-shazam),
the data-driven history-fingerprint match core. Build a `Shazam` from a spec
JSON, index an asset's history, and match the current fingerprint against it —
the same command protocol every language binding speaks.

## Install

```sh
pip install wickra-shazam
```

## Usage

```python
import json
from wickra_shazam import Shazam

spec = json.dumps({
    "features": [{"kind": "price", "field": "close"}],
    "window": 1,
    "metric": "euclid",
})

shazam = Shazam(spec)

def candle(time, close):
    return {"time": time, "open": close, "high": close,
            "low": close, "close": close, "volume": 1.0}

# Index the asset's history.
history = [candle(t, 100.0 + t) for t in range(1, 11)]
shazam.command(json.dumps({"cmd": "index", "history": history}))

# Match the current state against the history.
response = shazam.command(json.dumps({
    "cmd": "match",
    "current": [candle(11, 105.0)],
    "k": 3,
}))

report = json.loads(response)
print([m["ts"] for m in report["matches"]])
```

## API

| Method | Description |
|--------|-------------|
| `Shazam(spec_json)` | Build a shazam from a spec JSON (raises `ValueError` if invalid). |
| `shazam.command(cmd_json) -> str` | Apply a command JSON, return the response JSON. Commands: `set_spec`, `index`, `label`, `match`, `reset`, `version`. |
| `Shazam.version() -> str` | The library version. |

## Build from source

```sh
maturin develop --release
pytest -q
```

## License

`MIT OR Apache-2.0`.
