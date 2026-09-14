"""Operating-mode equivalence through the binding, over the golden corpus.

The command protocol offers two ways to attach a label to a historical window,
and both must yield the same ``match`` report bytes: a ``label`` sent before
``index`` is kept on the handle and applied when the index is built; a ``label``
sent after ``index`` goes onto the live index. Re-indexing the same history
keeps the labels and the report. The core pins this in Rust
(``operating_modes.rs``); this checks the boundary the Python binding crosses.
A missing corpus is a failure, not a skip.

Plain functions and plain asserts, so the module runs unchanged under pytest
(3.10 and up) and under ``run_without_pytest.py`` (the 3.9 row).
"""

import pathlib
import re

from wickra_shazam import Shazam

ROOT = pathlib.Path(__file__).resolve().parents[3]
GOLDEN = ROOT / "golden"
LABEL = "golden_top"


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


def test_a_label_before_index_matches_a_label_after_index() -> None:
    specs = sorted((GOLDEN / "specs").glob("*.json"))
    assert specs, "golden corpus not found"
    history = _candles_json(GOLDEN / "data/history/sym-01.csv")
    current = _candles_json(GOLDEN / "data/current/sym-01.csv")
    index_cmd = '{"cmd":"index","history":%s}' % history
    match_cmd = '{"cmd":"match","current":%s,"k":5}' % current
    for spec_path in specs:
        spec = spec_path.read_text(encoding="utf-8")
        expected = (GOLDEN / "expected" / spec_path.name).read_text(encoding="utf-8").strip()

        plain = Shazam(spec)
        plain.command(index_cmd)
        unlabelled = plain.command(match_cmd)
        assert unlabelled.strip() == expected, spec_path.name
        ts = re.search(r'"ts":(\d+)', unlabelled).group(1)
        label_cmd = '{"cmd":"label","ts":%s,"label":"%s"}' % (ts, LABEL)

        before = Shazam(spec)
        before.command(label_cmd)
        before.command(index_cmd)
        labelled_before = before.command(match_cmd)

        after = Shazam(spec)
        after.command(index_cmd)
        after.command(label_cmd)
        labelled_after = after.command(match_cmd)
        assert labelled_before == labelled_after, spec_path.name
        assert LABEL in labelled_after, spec_path.name
        assert labelled_after != unlabelled, spec_path.name

        after.command(index_cmd)
        assert after.command(match_cmd) == labelled_after, spec_path.name
