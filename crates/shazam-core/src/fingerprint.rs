//! The fingerprint vector and the rolling buffer that emits it.
//!
//! A [`Fingerprint`] is a fixed-length vector anchored to the timestamp of the
//! last bar in its window. A [`RollBuffer`] keeps the last `window` feature rows
//! and [`RollBuffer::emit`]s them **axis-major** (for each feature, its `window`
//! values in order) into a flat vector of length `features * window`. The layout
//! is fixed, so the flatten order is deterministic across every language and
//! both build profiles.

use std::collections::VecDeque;

pub use crate::spec::MAX_DIM;

/// One fingerprint: a fixed-length vector anchored to a timestamp. This is an
/// internal type — it never crosses the JSON language boundary.
#[derive(Clone, Debug, PartialEq)]
pub struct Fingerprint {
    /// Timestamp of the last bar of the window this fingerprint summarizes.
    pub ts: i64,
    /// The feature vector, length `features * window`.
    pub v: Box<[f64]>,
}

/// A ring buffer of the last `window` feature rows. Once full it emits a
/// fingerprint per bar.
pub struct RollBuffer {
    window: usize,
    rows: VecDeque<Vec<f64>>,
}

impl RollBuffer {
    /// A buffer holding up to `window` rows (`window >= 1`).
    #[must_use]
    pub fn new(window: usize) -> Self {
        Self {
            window,
            rows: VecDeque::with_capacity(window),
        }
    }

    /// Push a feature row, dropping the oldest once more than `window` are held.
    pub fn push(&mut self, row: Vec<f64>) {
        if self.rows.len() == self.window {
            self.rows.pop_front();
        }
        self.rows.push_back(row);
    }

    /// Whether the buffer holds a full window of rows.
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.rows.len() == self.window
    }

    /// Emit the current window as a flat, axis-major fingerprint: for each
    /// feature axis, its `window` values in time order. At `window == 1` this is
    /// the raw feature row. Assumes the buffer is full.
    #[must_use]
    pub fn emit(&self, ts: i64) -> Fingerprint {
        let features = self.rows.front().map_or(0, Vec::len);
        let mut v = Vec::with_capacity(features * self.window);
        for feature in 0..features {
            for row in &self.rows {
                v.push(row[feature]);
            }
        }
        Fingerprint {
            ts,
            v: v.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_one_is_the_raw_row() {
        let mut buf = RollBuffer::new(1);
        assert!(!buf.is_full());
        buf.push(vec![10.0, 20.0]);
        assert!(buf.is_full());
        let fp = buf.emit(7);
        assert_eq!(fp.ts, 7);
        assert_eq!(&*fp.v, &[10.0, 20.0]);
    }

    #[test]
    fn window_flatten_is_axis_major_and_deterministic() {
        let mut buf = RollBuffer::new(2);
        buf.push(vec![1.0, 2.0]);
        assert!(!buf.is_full());
        buf.push(vec![3.0, 4.0]);
        assert!(buf.is_full());
        // axis-major: feature 0 -> [1, 3], feature 1 -> [2, 4].
        let fp = buf.emit(9);
        assert_eq!(&*fp.v, &[1.0, 3.0, 2.0, 4.0]);
        assert_eq!(fp.v.len(), 4);
    }

    #[test]
    fn ring_drops_oldest() {
        let mut buf = RollBuffer::new(2);
        buf.push(vec![1.0]);
        buf.push(vec![2.0]);
        buf.push(vec![3.0]); // drops [1.0]
        let fp = buf.emit(0);
        assert_eq!(&*fp.v, &[2.0, 3.0]);
    }

    #[test]
    fn max_dim_is_exported() {
        assert_eq!(MAX_DIM, 4096);
    }
}
