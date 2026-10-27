//! Read hex-bit inputs/references from stdin. No dependencies.
use std::io::{self, BufRead};

fn bounded(y: f64, hot: f64, cold: f64) -> f64 {
    let numerator = (-y).mul_add(cold, hot);
    if numerator.is_finite() {
        numerator / (y - 1.0)
    } else {
        (hot - cold) / (y - 1.0) - cold
    }
}

fn selected(y: f64, hot: f64, cold: f64) -> f64 {
    rfconversions::noise::noise_temperature_from_y_factor(y, hot, cold)
}

fn main() {
    let mut counts = [0_usize; 6];
    let mut range_failures = [0_usize; 6];
    let mut worst_ulp = [0_u64; 6];
    let mut total = 0;
    for line in io::stdin().lock().lines() {
        let values: Vec<_> = line
            .unwrap()
            .split_whitespace()
            .map(|s| f64::from_bits(u64::from_str_radix(s, 16).unwrap()))
            .collect();
        let [y, hot, cold, expected] = values[..] else {
            panic!("expected four bit patterns")
        };
        let piecewise = if y <= 2.0 {
            (-y).mul_add(cold, hot) / (y - 1.0)
        } else {
            (hot - cold) / (y - 1.0) - cold
        };
        let results = [
            (hot - y * cold) / (y - 1.0),
            (hot - cold) / (y - 1.0) - cold,
            (-y).mul_add(cold, hot) / (y - 1.0),
            piecewise,
            bounded(y, hot, cold),
            selected(y, hot, cold),
        ];
        for (i, actual) in results.into_iter().enumerate() {
            if actual != expected {
                counts[i] += 1;
            }
            if (!actual.is_finite() || !expected.is_finite()) && actual != expected {
                range_failures[i] += 1;
            }
            if actual.is_finite() && expected.is_finite() {
                let ulps = if actual.signum() == expected.signum() {
                    actual.to_bits().abs_diff(expected.to_bits())
                } else {
                    actual
                        .abs()
                        .to_bits()
                        .saturating_add(expected.abs().to_bits())
                };
                worst_ulp[i] = worst_ulp[i].max(ulps);
            }
        }
        if total < 12 {
            println!("{total}: expected={expected:e} candidates={results:?}");
        }
        total += 1;
    }
    println!(
        "{total} cases; order: textbook, rearranged, fused, piecewise, bounded, scaled-bounded"
    );
    println!("not correctly rounded: {counts:?}");
    println!("nonfinite classification mismatches: {range_failures:?}");
    println!("worst finite ULP distance: {worst_ulp:?}");
    assert!(total > 0, "no reference cases supplied");
    assert_eq!(range_failures[5], 0, "public API range regression");
    assert!(worst_ulp[5] <= 1, "public API accuracy regression");
}
