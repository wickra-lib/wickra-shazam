//! Per-axis normalization of fingerprints across an index.
//!
//! Different feature axes have wildly different magnitudes (RSI 0–100 vs a price
//! of 60000). To compare them fairly, each axis is normalized over the whole
//! index. The per-axis parameters are computed once ([`fit_axes`]) and stored in
//! the index, then applied to both the historical fingerprints and the current
//! query fingerprint ([`apply`]) so the query lives in the same space as the
//! history. Reductions run **serially in (axis, ts) order** — never rayon order
//! — so the f64 rounding is identical everywhere.

use crate::fingerprint::Fingerprint;
use crate::spec::Normalize;

/// Per-axis statistics used to normalize a fingerprint. One per axis (length
/// `N`).
#[derive(Clone, Debug, PartialEq)]
pub struct AxisStats {
    /// Mean of the axis over the index.
    pub mean: f64,
    /// Population standard deviation of the axis (`sqrt(Σ(v-mean)²/m)`).
    pub std_pop: f64,
    /// Minimum value of the axis over the index.
    pub min: f64,
    /// Maximum value of the axis over the index.
    pub max: f64,
}

/// Compute the per-axis statistics over `prints` for an `n`-dimensional
/// fingerprint. Iterates axis-by-axis, and within an axis in fingerprint (ts)
/// order, for deterministic reduction. Empty input yields all-zero stats.
#[must_use]
pub fn fit_axes(prints: &[Fingerprint], n: usize) -> Vec<AxisStats> {
    let m = prints.len();
    let mut axes = Vec::with_capacity(n);
    for axis in 0..n {
        if m == 0 {
            axes.push(AxisStats {
                mean: 0.0,
                std_pop: 0.0,
                min: 0.0,
                max: 0.0,
            });
            continue;
        }
        let mut sum = 0.0;
        let mut min = f64::INFINITY;
        let mut max = f64::NEG_INFINITY;
        for print in prints {
            let x = print.v[axis];
            sum += x;
            if x < min {
                min = x;
            }
            if x > max {
                max = x;
            }
        }
        let mean = sum / m as f64;
        let mut var = 0.0;
        for print in prints {
            let d = print.v[axis] - mean;
            var += d * d;
        }
        let std_pop = (var / m as f64).sqrt();
        axes.push(AxisStats {
            mean,
            std_pop,
            min,
            max,
        });
    }
    axes
}

/// Normalize a fingerprint vector in place using the stored axis stats. Degenerate
/// axes are clamped so no `NaN`/`±inf` reaches the output: a zero standard
/// deviation (z-score) or a zero range (min-max) maps the axis to `0.0`.
pub fn apply(v: &mut [f64], axes: &[AxisStats], mode: Normalize) {
    match mode {
        Normalize::None => {}
        Normalize::ZScore => {
            for (x, axis) in v.iter_mut().zip(axes) {
                *x = if axis.std_pop == 0.0 {
                    0.0
                } else {
                    (*x - axis.mean) / axis.std_pop
                };
            }
        }
        Normalize::MinMax => {
            for (x, axis) in v.iter_mut().zip(axes) {
                let range = axis.max - axis.min;
                *x = if range == 0.0 {
                    0.0
                } else {
                    (*x - axis.min) / range
                };
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fp(v: &[f64]) -> Fingerprint {
        Fingerprint {
            ts: 0,
            v: v.to_vec().into_boxed_slice(),
        }
    }

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-12
    }

    #[test]
    fn fits_mean_std_min_max() {
        let prints = [fp(&[1.0]), fp(&[3.0])];
        let axes = fit_axes(&prints, 1);
        assert!(close(axes[0].mean, 2.0));
        assert!(close(axes[0].std_pop, 1.0)); // sqrt(((1-2)^2 + (3-2)^2)/2) = 1
        assert!(close(axes[0].min, 1.0));
        assert!(close(axes[0].max, 3.0));
    }

    #[test]
    fn zscore_normalizes_and_clamps_degenerate() {
        let axes = vec![
            AxisStats {
                mean: 2.0,
                std_pop: 1.0,
                min: 1.0,
                max: 3.0,
            },
            AxisStats {
                mean: 5.0,
                std_pop: 0.0, // degenerate: every value equal
                min: 5.0,
                max: 5.0,
            },
        ];
        let mut v = vec![3.0, 5.0];
        apply(&mut v, &axes, Normalize::ZScore);
        assert_eq!(v, vec![1.0, 0.0]);
    }

    #[test]
    fn minmax_normalizes_and_clamps_degenerate() {
        let axes = vec![
            AxisStats {
                mean: 0.0,
                std_pop: 0.0,
                min: 0.0,
                max: 10.0,
            },
            AxisStats {
                mean: 7.0,
                std_pop: 0.0,
                min: 7.0,
                max: 7.0, // degenerate: max == min
            },
        ];
        let mut v = vec![5.0, 7.0];
        apply(&mut v, &axes, Normalize::MinMax);
        assert_eq!(v, vec![0.5, 0.0]);
    }

    #[test]
    fn none_is_passthrough() {
        let axes = fit_axes(&[fp(&[1.0])], 1);
        let mut v = vec![42.0];
        apply(&mut v, &axes, Normalize::None);
        assert_eq!(v, vec![42.0]);
    }

    #[test]
    fn empty_prints_yield_zero_stats() {
        let axes = fit_axes(&[], 2);
        assert_eq!(axes.len(), 2);
        assert!(close(axes[0].std_pop, 0.0));
    }
}
