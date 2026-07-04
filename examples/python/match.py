"""A runnable Python example: index a history and match the current state.

    pip install wickra-shazam
    python examples/python/match.py
"""

import json

from wickra_shazam import Shazam

SPEC = json.dumps(
    {
        "features": [{"kind": "price", "field": "close"}],
        "window": 1,
        "metric": "euclid",
    }
)


def candle(time: int, close: float) -> dict:
    return {
        "time": time,
        "open": close,
        "high": close,
        "low": close,
        "close": close,
        "volume": 1.0,
    }


def main() -> None:
    shazam = Shazam(SPEC)
    shazam.command(
        json.dumps(
            {"cmd": "index", "history": [candle(1, 100.0), candle(2, 101.0), candle(3, 102.0)]}
        )
    )
    response = shazam.command(
        json.dumps({"cmd": "match", "current": [candle(4, 102.0)], "k": 2})
    )
    report = json.loads(response)

    print(f"wickra-shazam {Shazam.version()}")
    print(response)
    for match in report["matches"]:
        print(f"  match ts {match['ts']} similarity {match['similarity']}")


if __name__ == "__main__":
    main()
