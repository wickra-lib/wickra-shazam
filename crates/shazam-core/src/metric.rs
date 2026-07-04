//! Similarity metrics: distance turned into a `[0, 1]` similarity (1 = identical).
//!
//! All reductions run **serially in axis order** — never rayon order — so the
//! f64 rounding is identical across every language and both build profiles.
//! Cosine and Euclid operate on the flat fingerprint vector; DTW operates on the
//! window as a 2D sequence (one feature-vector per time step) with a Sakoe-Chiba
//! band. Degenerate cases are clamped so no `NaN`/`±inf` reaches the output.

use crate::spec::Metric;

/// Similarity between two flat fingerprint vectors under `metric`. Cosine and
/// Euclid are handled here; a `Dtw` spec with `window == 1` is identical to
/// Euclid, and a wider DTW window is routed to [`dtw_similarity`].
#[must_use]
pub fn similarity(a: &[f64], b: &[f64], metric: Metric) -> f64 {
    match metric {
        Metric::Cosine => cosine(a, b),
        Metric::Euclid | Metric::Dtw => euclid(a, b),
    }
}

/// Cosine similarity mapped from `[-1, 1]` to `[0, 1]` via `(cos + 1) / 2`. A
/// zero-norm vector yields `cos = 0` (similarity `0.5`).
fn cosine(a: &[f64], b: &[f64]) -> f64 {
    let mut dot = 0.0;
    let mut na = 0.0;
    let mut nb = 0.0;
    for (x, y) in a.iter().zip(b) {
        dot += x * y;
        na += x * x;
        nb += y * y;
    }
    let norm = na.sqrt() * nb.sqrt();
    let cos = if norm == 0.0 { 0.0 } else { dot / norm };
    cos.midpoint(1.0)
}

/// Euclidean similarity: `1 / (1 + d)` where `d` is the L2 distance.
fn euclid(a: &[f64], b: &[f64]) -> f64 {
    1.0 / (1.0 + euclid_dist(a, b))
}

/// L2 distance between two equal-length vectors.
fn euclid_dist(a: &[f64], b: &[f64]) -> f64 {
    let mut sum = 0.0;
    for (x, y) in a.iter().zip(b) {
        let d = x - y;
        sum += d * d;
    }
    sum.sqrt()
}

/// DTW similarity between two sequences of feature-vectors (one per time step)
/// under a Sakoe-Chiba band, mapped via `1 / (1 + d)`. The local cost per cell
/// pair is the Euclidean distance over the feature axes. The DP table is filled
/// row-major with a fixed iteration order. Assumes both sequences are non-empty.
#[must_use]
pub fn dtw_similarity(a: &[&[f64]], b: &[&[f64]], band: usize) -> f64 {
    let n = a.len();
    let m = b.len();
    let mut dp = vec![vec![f64::INFINITY; m]; n];
    for i in 0..n {
        let lo = i.saturating_sub(band);
        let hi = (i + band + 1).min(m);
        for j in lo..hi {
            let cost = euclid_dist(a[i], b[j]);
            let prev = if i == 0 && j == 0 {
                0.0
            } else {
                let mut best = f64::INFINITY;
                if i > 0 {
                    best = best.min(dp[i - 1][j]);
                }
                if j > 0 {
                    best = best.min(dp[i][j - 1]);
                }
                if i > 0 && j > 0 {
                    best = best.min(dp[i - 1][j - 1]);
                }
                best
            };
            dp[i][j] = cost + prev;
        }
    }
    1.0 / (1.0 + dp[n - 1][m - 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn identical_vectors_are_maximally_similar() {
        let v = [1.0, 2.0, 3.0];
        assert!(close(similarity(&v, &v, Metric::Cosine), 1.0));
        assert!(close(similarity(&v, &v, Metric::Euclid), 1.0));
    }

    #[test]
    fn orthogonal_cosine_is_a_half() {
        let a = [1.0, 0.0];
        let b = [0.0, 1.0];
        assert!(close(similarity(&a, &b, Metric::Cosine), 0.5));
    }

    #[test]
    fn zero_norm_cosine_is_a_half() {
        let a = [0.0, 0.0];
        let b = [1.0, 2.0];
        assert!(close(similarity(&a, &b, Metric::Cosine), 0.5));
    }

    #[test]
    fn euclid_is_monotone_in_distance() {
        let base = [0.0, 0.0];
        let near = [1.0, 0.0];
        let far = [3.0, 0.0];
        let s_near = similarity(&base, &near, Metric::Euclid);
        let s_far = similarity(&base, &far, Metric::Euclid);
        assert!(s_near > s_far);
        assert!(s_near <= 1.0 && s_far >= 0.0);
    }

    #[test]
    fn dtw_equals_euclid_at_window_one() {
        let a_flat = [1.0, 2.0];
        let b_flat = [4.0, 6.0];
        let a_seq: [&[f64]; 1] = [&a_flat];
        let b_seq: [&[f64]; 1] = [&b_flat];
        let dtw = dtw_similarity(&a_seq, &b_seq, 1);
        let euc = similarity(&a_flat, &b_flat, Metric::Euclid);
        assert!(close(dtw, euc));
    }

    #[test]
    fn dtw_identical_sequences_are_maximally_similar() {
        let r0 = [1.0, 2.0];
        let r1 = [3.0, 4.0];
        let seq: [&[f64]; 2] = [&r0, &r1];
        assert!(close(dtw_similarity(&seq, &seq, 2), 1.0));
    }
}
