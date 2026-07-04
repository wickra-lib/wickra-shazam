"""Wickra Shazam — the data-driven history-fingerprint match core.

Build a :class:`Shazam` from a spec JSON, index an asset's history, and match
the current fingerprint against it. The same command protocol crosses every
language binding, so this Python front-end drives the exact same core as the
native CLI.
"""

from ._wickra_shazam import Shazam, __version__

__all__ = ["Shazam", "__version__"]
