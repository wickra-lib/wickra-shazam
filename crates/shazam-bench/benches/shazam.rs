//! Criterion benchmarks for the shazam core.
//!
//! `build_index` scales by history length (1k / 10k / 100k bars) and feature
//! count (5 / 20 indicators); `match_index` scales by index size (10k / 100k
//! fingerprints) across the three metrics (cosine / euclid / DTW). The same
//! benchmarks, run with and without the `parallel` feature, measure the rayon
//! path against the sequential one.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use shazam_core::{build_index, match_index, Candle, FingerprintSpec};

const INDICATORS: [&str; 6] = ["Rsi", "Roc", "Sma", "Ema", "Atr", "Mfi"];

/// A deterministic OHLCV history of `bars` bars (a per-bar sine path plus drift).
fn history(bars: usize) -> Vec<Candle> {
    (0..bars)
        .map(|i| {
            let t = i as f64;
            let close = 100.0 + 12.0 * (t * 0.05).sin() + 0.01 * t;
            let open = 100.0 + 12.0 * ((t - 0.5) * 0.05).sin() + 0.01 * t;
            Candle {
                time: 1_700_000_000 + i64::try_from(i).unwrap() * 3600,
                open,
                high: open.max(close) + 0.5,
                low: open.min(close) - 0.5,
                close,
                volume: 1000.0 + t,
            }
        })
        .collect()
}

/// A spec whose `n` features cycle through single-parameter indicators (so the
/// per-bar fold touches `n` distinct evaluators).
fn indicator_spec(n: usize, window: usize, normalize: &str) -> FingerprintSpec {
    let features: Vec<String> = (0..n)
        .map(|i| {
            let name = INDICATORS[i % INDICATORS.len()];
            let period = 5 + i / INDICATORS.len();
            format!(r#"{{"kind":"indicator","name":"{name}","params":[{period}]}}"#)
        })
        .collect();
    let json = format!(
        r#"{{"features":[{}],"window":{window},"normalize":"{normalize}","metric":"cosine"}}"#,
        features.join(",")
    );
    FingerprintSpec::from_json(&json).unwrap()
}

/// A price-only spec on a given metric (isolates the match/distance cost).
fn price_spec(metric: &str, window: usize) -> FingerprintSpec {
    let json = format!(
        r#"{{"features":[{{"kind":"price","field":"close"}},{{"kind":"price","field":"high"}}],"window":{window},"normalize":"z_score","metric":"{metric}"}}"#
    );
    FingerprintSpec::from_json(&json).unwrap()
}

fn bench_build_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("build_index");
    group.sample_size(10);
    for &bars in &[1_000usize, 10_000, 100_000] {
        let data = history(bars);
        for &features in &[5usize, 20] {
            let spec = indicator_spec(features, 8, "z_score");
            group.throughput(Throughput::Elements(bars as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{bars}bars/{features}feat")),
                &(&data, &spec),
                |b, (data, spec)| b.iter(|| build_index(black_box(data), black_box(spec)).unwrap()),
            );
        }
    }
    group.finish();
}

fn bench_match_index(c: &mut Criterion) {
    let mut group = c.benchmark_group("match_index");
    group.sample_size(10);
    let window = 8;
    for &prints in &[10_000usize, 100_000] {
        let data = history(prints + window);
        let current = data[data.len() - 2 * window..].to_vec();
        for metric in ["cosine", "euclid", "dtw"] {
            let spec = price_spec(metric, window);
            let index = build_index(&data, &spec).unwrap();
            group.throughput(Throughput::Elements(prints as u64));
            group.bench_with_input(
                BenchmarkId::from_parameter(format!("{prints}prints/{metric}")),
                &(&index, &current),
                |b, (index, current)| {
                    b.iter(|| match_index(black_box(index), black_box(current), 10).unwrap());
                },
            );
        }
    }
    group.finish();
}

criterion_group!(benches, bench_build_index, bench_match_index);
criterion_main!(benches);
