//! Public-API references from exact f64 inputs using Decimal at 500 and 800
//! digits; see docs/noise-conversion-accuracy.md for regeneration and limits.
use rfconversions::noise::{
    noise_figure_from_noise_temperature as to_nf, noise_temperature_from_noise_figure as to_te,
};

fn relative(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() / expected.abs() <= tolerance,
        "actual={actual:.17e}, expected={expected:.17e}"
    );
}

#[test]
fn ordinary_and_tiny_nonzero_conversions() {
    for (input, expected) in [
        (0.5, 35.385351747569395),
        (3.0, 288.6260713409751),
        (6.0, 864.510794605142),
        (1e-16, 6.677496769682733e-15),
        (-1e-16, -6.677496769682732e-15),
        (f64::EPSILON, 1.4827021321123753e-14),
        (-f64::EPSILON, -1.482702132112375e-14),
    ] {
        relative(to_te(input), expected, 2e-15);
    }
    for (input, expected) in [
        (35.0, 0.49485363079918293),
        (290.0, 3.010299956639812),
        (1e-16, 1.4975671789767304e-18),
        (-1e-16, -1.4975671789767304e-18),
        (290.0 * f64::EPSILON, 9.64327466553287e-16),
        (-290.0 * f64::EPSILON, -9.643274665532871e-16),
        (-145.0, -3.010299956639812),
        (-145.00000000000003, -3.010299956639813),
        (f64::MAX, 3057.923175620178),
    ] {
        relative(to_nf(input), expected, 2e-15);
    }
}

#[test]
fn subnormal_rescaling_does_not_lose_representable_results() {
    let smallest = f64::from_bits(1);
    for sign in [-1.0, 1.0] {
        assert_eq!(to_te(sign * smallest), sign * f64::from_bits(67));
        assert_eq!(to_te(sign * 1e-320), sign * 6.67745e-319);
        assert_eq!(to_nf(sign * 2e-322), sign * smallest);
        assert_eq!(to_nf(sign * 1e-320), sign * f64::from_bits(30));
    }
    assert_eq!(to_nf(smallest).to_bits(), 0.0_f64.to_bits());
    assert_eq!(to_nf(-smallest).to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn algebraic_domain_and_nonfinite_values() {
    let above = f64::from_bits((-290.0_f64).to_bits() - 1);
    let below = f64::from_bits((-290.0_f64).to_bits() + 1);
    relative(to_nf(above), -157.07717807114128, 2e-15);
    assert_eq!(to_nf(-290.0), f64::NEG_INFINITY);
    assert!(to_nf(below).is_nan());
    assert!(to_nf(f64::NEG_INFINITY).is_nan());
    assert_eq!(to_nf(f64::INFINITY), f64::INFINITY);
    assert_eq!(to_te(f64::NEG_INFINITY), -290.0);
    assert_eq!(to_te(f64::INFINITY), f64::INFINITY);
    assert!(to_te(f64::NAN).is_nan());
    assert!(to_nf(f64::NAN).is_nan());
    for zero in [0.0, -0.0] {
        assert_eq!(to_te(zero).to_bits(), 0.0_f64.to_bits());
        assert_eq!(to_nf(zero).to_bits(), 0.0_f64.to_bits());
    }
}

#[test]
fn large_finite_temperature_and_overflow_boundary() {
    // The neighboring input above this finite case has an overflowing reference.
    relative(to_te(3057.9231756201775), 1.7976931348621664e308, 3e-13);
    assert_eq!(to_te(3057.923175620178), f64::INFINITY);
    assert_eq!(to_te(4000.0), f64::INFINITY);
    assert_eq!(to_te(-4000.0), -290.0);
}
