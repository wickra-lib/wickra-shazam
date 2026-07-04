# Labels

A **label** attaches a human-readable name to a historical timestamp. When a
match lands on that bar, the name rides back in the report — so instead of
"closest match at ts 1700216000" you get "closest match: `may_2021_crash`".
Labels are pure annotation: they never affect which fingerprints match or their
similarity, only how a match reads.

## Attaching a label

Over the command protocol, `label` maps a timestamp to a name:

```jsonc
{ "cmd": "label", "ts": 1700216000, "label": "may_2021_crash" }
```

On the CLI, `--label <ts>=<name>` does the same and is repeatable:

```bash
cargo run -p wickra-shazam -- \
  --spec golden/specs/crash_setup.json \
  --history golden/data/history/sym-01.csv \
  --label 1700216000=may_2021_crash \
  --label 1700188000=summer_range \
  --format json
```

## How it comes back

A labelled bar carries its name in the match; unlabelled bars simply omit the
field (the `label` key is skipped when absent, keeping the JSON compact):

```json
{
  "matches": [
    { "ts": 1700216000, "similarity": 0.982, "label": "may_2021_crash" },
    { "ts": 1700120000, "similarity": 0.774 }
  ],
  "indexed": 512
}
```

## Notes

- Labels attach to a **timestamp**, not to a fingerprint, so they survive a
  `reset` + re-`index` as long as the same bar is present.
- Re-labelling the same timestamp replaces the previous name.
- A label on a timestamp that never appears in a match is simply never shown; it
  is not an error.

## See also

- [FINGERPRINTS.md](FINGERPRINTS.md) · [FEATURES.md](FEATURES.md) · [SIMILARITY.md](SIMILARITY.md) · [Cookbook.md](Cookbook.md) · [ARCHITECTURE.md](ARCHITECTURE.md)
