//! Public API checks with independent Decimal references; see docs/power-addition.md.
use rfconversions::power::add_power_db;

#[test]
fn finite_power_sums_against_independent_references() {
    // Absolute dB bounds: tiny corrections need a much tighter bound than
    // outputs near zero formed by cancellation of order-one terms.
    for (a, b, expected, tolerance) in [
        (0.0, 0.0, 3.010299956639812, 1e-14),
        (0.0, -160.0, 4.342944819032518e-16, 1e-29),
        (4000.0, 4000.0, 4003.01029995664, 1e-12),
        (-4000.0, -4000.0, -3996.98970004336, 1e-12),
        (-100.0, -110.0, -99.58607314841775, 3e-14),
        (
            -3.010299956639812,
            -3.010299956639812,
            -1.3905958374036735e-16,
            1e-15,
        ),
    ] {
        for (a, b) in [(a, b), (b, a)] {
            let actual = add_power_db(a, b);
            assert!(
                (actual - expected).abs() <= tolerance,
                "add_power_db({a}, {b}) = {actual}, expected {expected} ± {tolerance} dB"
            );
        }
    }
}

#[test]
fn zero_power_is_an_identity_including_signed_zero() {
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
}

#[test]
fn nan_precedes_infinity_and_infinity_precedes_finite_power() {
    for value in [f64::NEG_INFINITY, -100.0, 0.0, f64::INFINITY, f64::NAN] {
        assert!(add_power_db(value, f64::NAN).is_nan());
        assert!(add_power_db(f64::NAN, value).is_nan());
        if !value.is_nan() {
            assert_eq!(add_power_db(value, f64::INFINITY), f64::INFINITY);
            assert_eq!(add_power_db(f64::INFINITY, value), f64::INFINITY);
        }
    }
}

#[test]
fn negligible_terms_and_final_result_rounding_at_extreme_finite_levels() {
    // Even a negatively overflowing shift only discards negligible power.
    assert_eq!(add_power_db(f64::MAX, -f64::MAX), f64::MAX);
    assert_eq!(add_power_db(-f64::MAX, f64::MAX), f64::MAX);
    assert_eq!(add_power_db(1000.0, -4000.0), 1000.0);
    for level in [f64::MAX, -f64::MAX, 1e20, -1e20] {
        // A ~3 dB increase is smaller than half an output ULP here.
        assert_eq!(add_power_db(level, level), level);
    }
}
