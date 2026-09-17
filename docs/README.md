# Documentation

The reference documentation for Wickra Shazam lives at
**[shazam.wickra.org](https://shazam.wickra.org)** — quickstarts, the API
surface for every binding, and the guides.

What stays here, beside the code, is the material that only makes sense next to
the implementation and has to change in the same commit as it:

- [`ARCHITECTURE.md`](ARCHITECTURE.md)
- [`Cookbook.md`](Cookbook.md)
- [`FEATURES.md`](FEATURES.md)
- [`FINGERPRINTS.md`](FINGERPRINTS.md)
- [`LABELS.md`](LABELS.md)
- [`SIMILARITY.md`](SIMILARITY.md)

The per-binding READMEs under `bindings/` describe each package as its registry
page shows it.

## Editing the docs

The documentation site is a separate git repository at
`https://github.com/wickra-lib/wickra-shazam-site`. Open a pull request there to
propose changes; the site is built with VitePress and deploys to
`shazam.wickra.org`. The files in this directory change in the same commit as
the code they describe.
