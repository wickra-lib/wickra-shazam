"""Smoke test: construct a shazam, index a history, match, parse the report."""

import json

from wickra_shazam import Shazam, __version__

SPEC = json.dumps(
    {
        "features": [{"kind": "price", "field": "close"}],
        "window": 1,
        "metric": "euclid",
    }
)


def _candle(time: int, close: float) -> dict:
    return {
        "time": time,
        "open": close,
        "high": close,
        "low": close,
        "close": close,
        "volume": 1.0,
    }


def test_index_and_match_roundtrip() -> None:
    shazam = Shazam(SPEC)
    history = [_candle(t, 100.0 + t) for t in range(1, 11)]
    indexed = json.loads(
        shazam.command(json.dumps({"cmd": "index", "history": history}))
    )
    assert indexed["indexed"] == 10

    response = shazam.command(
        json.dumps({"cmd": "match", "current": [_candle(11, 110.0)], "k": 3})
    )
    report = json.loads(response)
    assert report["indexed"] == 10
    assert len(report["matches"]) == 3
    # The tail of a monotone history is most similar to its latest bars.
    assert report["matches"][0]["ts"] == 10


def test_version_matches_module() -> None:
    assert Shazam.version() == __version__


def test_bad_spec_raises() -> None:
    try:
        Shazam("not json")
    except ValueError:
        return
    raise AssertionError("expected ValueError for a malformed spec")
