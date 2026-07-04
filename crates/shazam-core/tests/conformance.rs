//! Conformance: serde round-trips of every spec enum, canonical `Feature::key`
//! snapshots, and the structural validation boundaries (unknown feature/field,
//! dimension over `MAX_DIM`).

use shazam_core::{
    build_index, Candle, Error, Feature, FingerprintSpec, Metric, Normalize, PriceField,
};

/// Round-trip a value through JSON and assert the re-serialization is stable.
fn roundtrip<T>(value: &T)
where
    T: serde::Serialize + serde::de::DeserializeOwned,
{
    let json = serde_json::to_string(value).unwrap();
    let back: T = serde_json::from_str(&json).unwrap();
    assert_eq!(serde_json::to_string(&back).unwrap(), json);
}

fn candle(time: i64, close: f64) -> Candle {
    serde_json::from_str(&format!(
        r#"{{"time":{time},"open":{close},"high":{close},"low":{close},"close":{close},"volume":1}}"#
    ))
    .unwrap()
}

#[test]
fn price_field_variants_round_trip() {
    for field in [
        PriceField::Open,
        PriceField::High,
        PriceField::Low,
        PriceField::Close,
        PriceField::Volume,
    ] {
        roundtrip(&field);
    }
}

#[test]
fn normalize_variants_round_trip() {
    for norm in [Normalize::None, Normalize::ZScore, Normalize::MinMax] {
        roundtrip(&norm);
    }
}

#[test]
fn metric_variants_round_trip() {
    for metric in [Metric::Cosine, Metric::Euclid, Metric::Dtw] {
        roundtrip(&metric);
    }
}

#[test]
fn feature_variants_round_trip() {
    let features = [
        Feature::Price {
            field: PriceField::Close,
        },
        Feature::Indicator {
            name: "Rsi".into(),
            params: vec![14.0],
            field: None,
        },
        Feature::Microstructure {
            name: "Obv".into(),
            params: vec![],
            field: Some("close".into()),
        },
    ];
    for feature in &features {
        roundtrip(feature);
    }
}

#[test]
fn feature_keys_are_canonical() {
    assert_eq!(
        Feature::Price {
            field: PriceField::Close
        }
        .key(),
        "price.close"
    );
    assert_eq!(
        Feature::Indicator {
            name: "Rsi".into(),
            params: vec![14.0],
            field: None,
        }
        .key(),
        "Rsi(14)"
    );
    assert_eq!(
        Feature::Indicator {
            name: "Roc".into(),
            params: vec![10.0],
            field: Some("close".into()),
        }
        .key(),
        "Roc(10).close"
    );
    assert_eq!(
        Feature::Microstructure {
            name: "Obv".into(),
            params: vec![],
            field: None,
        }
        .key(),
        "micro.Obv()"
    );
}

#[test]
fn unknown_price_field_fails_to_parse() {
    let json = r#"{"features":[{"kind":"price","field":"bogus"}],"window":1}"#;
    assert!(FingerprintSpec::from_json(json).is_err());
}

#[test]
fn unknown_indicator_is_an_unknown_feature() {
    let spec = FingerprintSpec::from_json(
        r#"{"features":[{"kind":"indicator","name":"NotAnIndicator","params":[]}],"window":1}"#,
    )
    .unwrap();
    let history: Vec<Candle> = (0..5)
        .map(|i| candle(1_700_000_000 + i, 100.0 + i as f64))
        .collect();
    // `FingerprintIndex` is intentionally not `Debug`, so match the `Result`.
    assert!(matches!(
        build_index(&history, &spec),
        Err(Error::UnknownFeature(_))
    ));
}

#[test]
fn dimension_over_max_dim_is_a_bad_spec() {
    // dim = features.len() * window = 1 * 5000 > MAX_DIM (4096).
    let json = r#"{"features":[{"kind":"price","field":"close"}],"window":5000}"#;
    assert!(matches!(
        FingerprintSpec::from_json(json),
        Err(Error::BadSpec(_))
    ));
}

#[test]
fn empty_features_is_a_bad_spec() {
    assert!(matches!(
        FingerprintSpec::from_json(r#"{"features":[],"window":1}"#),
        Err(Error::BadSpec(_))
    ));
}
