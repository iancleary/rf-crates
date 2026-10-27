//! Read Decimal-generated CSV from stdin and compare three finite algorithms.
use rfconversions::power::{add_power_db, db_to_linear, linear_to_db};
use std::io::{self, BufRead};

fn main() {
    let mut failures = [0; 3];
    println!("a,b,reference,composition,shift_log10,production_ln_1p");
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let values: Vec<f64> = line.split(',').map(|v| v.parse().unwrap()).collect();
        let [a, b, expected, tolerance] = values[..] else {
            panic!("expected a,b,reference,tolerance");
        };
        for (a, b) in [(a, b), (b, a)] {
            let high = a.max(b);
            let low = a.min(b);
            let results = [
                linear_to_db(db_to_linear(a) + db_to_linear(b)),
                high + 10.0 * (1.0 + 10.0_f64.powf((low - high) / 10.0)).log10(),
                add_power_db(a, b),
            ];
            println!(
                "{a},{b},{expected:.17e},{:.17e},{:.17e},{:.17e}",
                results[0], results[1], results[2]
            );
            for (i, result) in results.into_iter().enumerate() {
                if !result.is_finite() || (result - expected).abs() > tolerance {
                    failures[i] += 1;
                }
            }
        }
    }
    println!("Failed finite samples (including swaps): {failures:?}");
    assert!(failures[0] > 0 && failures[1] > 0);
    assert_eq!(failures[2], 0);

    // Exact classifications and identities, not transcendental reference points.
    for value in [f64::NEG_INFINITY, -100.0, -0.0, 0.0, f64::INFINITY] {
        assert_eq!(
            add_power_db(value, f64::NEG_INFINITY).to_bits(),
            value.to_bits()
        );
        assert_eq!(
            add_power_db(f64::NEG_INFINITY, value).to_bits(),
            value.to_bits()
        );
    }
    for value in [f64::NEG_INFINITY, -100.0, 0.0, f64::INFINITY, f64::NAN] {
        assert!(add_power_db(value, f64::NAN).is_nan());
        assert!(add_power_db(f64::NAN, value).is_nan());
        if !value.is_nan() {
            assert_eq!(add_power_db(value, f64::INFINITY), f64::INFINITY);
            assert_eq!(add_power_db(f64::INFINITY, value), f64::INFINITY);
        }
    }
    // The negative shift overflows; its negligible contribution underflows to zero.
    assert_eq!(add_power_db(f64::MAX, -f64::MAX), f64::MAX);
    assert_eq!(add_power_db(-f64::MAX, f64::MAX), f64::MAX);
    // At these magnitudes even doubling power rounds back to the input dB value.
    for level in [f64::MAX, -f64::MAX, 1e20, -1e20] {
        assert_eq!(add_power_db(level, level), level);
    }
    println!("Production boundary and extreme-finite checks passed");
}
