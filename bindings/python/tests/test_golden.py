"""Cross-language golden: index sym-01's history, match its current window, and
assert the MatchReport JSON is byte-identical to golden/expected/*.json.

Candle JSON is built from the raw CSV tokens so no per-language number formatting
can drift. The fixtures live in the repository-root ``golden/`` directory. A
missing corpus is a failure, not a skip.

Plain functions and plain asserts, so the module runs unchanged under pytest
(3.10 and up) and under ``run_without_pytest.py`` (the 3.9 row).
"""

import pathlib

from wickra_shazam import Shazam

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"


def _candles_json(path: pathlib.Path) -> str:
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        c = line.split(",")
        try:
            int(c[0])
        except ValueError:
            continue  # header
        rows.append(
            '{"time":%s,"open":%s,"high":%s,"low":%s,"close":%s,"volume":%s}'
            % (c[0], c[1], c[2], c[3], c[4], c[5])
        )
    return "[" + ",".join(rows) + "]"


def test_golden_match_is_byte_identical() -> None:
    specs = sorted((GOLDEN / "specs").glob("*.json"))
    assert specs, "golden corpus not found"
    history = _candles_json(GOLDEN / "data/history/sym-01.csv")
    current = _candles_json(GOLDEN / "data/current/sym-01.csv")
    for spec_path in specs:
        expected = (GOLDEN / "expected" / spec_path.name).read_text(encoding="utf-8").strip()
        shazam = Shazam(spec_path.read_text(encoding="utf-8"))
        shazam.command('{"cmd":"index","history":%s}' % history)
        response = shazam.command('{"cmd":"match","current":%s,"k":5}' % current)
        assert response.strip() == expected, spec_path.name
