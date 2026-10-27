//! Independent analytic and exact-rational calibration expectations.
use rfconversions::noise::noise_temperature_from_y_factor as temperature;

#[test]
fn analytic_calibrations_and_negative_diagnostic() {
    for (y, hot, cold, expected) in [
        (2.0, 290.0, 77.0, 136.0),
        (2.0, 290.0, 0.0, 290.0),
        (3.0, 300.0, 100.0, 0.0),
        (5.0, 300.0, 100.0, -50.0),
    ] {
        assert_eq!(temperature(y, hot, cold), expected);
    }
    assert_eq!(temperature(2.0, 290.0, -0.0), 290.0);
}

#[test]
fn rejects_invalid_arguments_in_each_position() {
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(temperature(invalid, 290.0, 77.0).is_nan());
        assert!(temperature(2.0, invalid, 77.0).is_nan());
        assert!(temperature(2.0, 290.0, invalid).is_nan());
    }
    for y in [-1.0, -0.0, 0.0, 1.0, f64::from_bits(1.0_f64.to_bits() - 1)] {
        assert!(temperature(y, 290.0, 77.0).is_nan());
    }
    for (hot, cold) in [
        (77.0, 77.0),
        (76.0, 77.0),
        (0.0, 0.0),
        (-1.0, 0.0),
        (290.0, -1.0),
    ] {
        assert!(temperature(2.0, hot, cold).is_nan());
    }
}

#[test]
fn preserves_finite_results_across_extreme_intermediates() {
    // The rearranged quotient overflows here although Te is finite.
    assert_eq!(temperature(1.5, 1.5e308, 5e307), 1.5e308);
    // The fused numerator overflows here although Te approaches -Tc.
    assert_eq!(temperature(1e308, 300.0, 100.0), -100.0);
    assert_eq!(temperature(f64::MAX, f64::MAX, 0.0), 1.0);
    assert_eq!(
        temperature(f64::MAX, f64::MAX, f64::MAX / 2.0),
        -f64::MAX / 2.0
    );
    let near_one = f64::from_bits(1.0_f64.to_bits() + 1);
    assert_eq!(temperature(near_one, 1.7e308, 0.0), f64::INFINITY);
}

#[test]
fn cancellation_and_subnormal_references() {
    // Rounded exact-Fraction references from the bundled offline generator.
    let near_one = f64::from_bits(1.0_f64.to_bits() + 1);
    let above_three = f64::from_bits(3.0_f64.to_bits() + 1);
    assert_eq!(temperature(near_one, 290.0, 77.0), 9.592667206299155e17);
    assert_eq!(
        temperature(above_three, 300.0, 100.0),
        -2.2204460492503124e-14
    );
    // Th = Y*Tc exactly: the equal-temperature-neighbor inputs matter.
    assert_eq!(temperature(near_one, near_one, 1.0), 0.0);
    let tiny = f64::from_bits(1);
    assert_eq!(temperature(1.5, 2.0 * tiny, tiny), tiny);
    assert_eq!(
        temperature(near_one, 2.0 * tiny, tiny),
        f64::from_bits(0x000fffffffffffff)
    );
}
