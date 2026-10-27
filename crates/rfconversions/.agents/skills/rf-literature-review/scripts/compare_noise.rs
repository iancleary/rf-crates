//! Read noise_references.py CSV on stdin; compare baseline, direct, range-aware and public APIs.
use std::io::{self, BufRead};

fn ranged_nf(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x.abs() < f64::EPSILON {
        x * (290.0 * (std::f64::consts::LN_10 / 10.0))
    } else {
        290.0 * (x * (std::f64::consts::LN_10 / 10.0)).exp_m1()
    }
}

fn ranged_te(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x.abs() < 290.0 * f64::EPSILON {
        x * ((10.0 / std::f64::consts::LN_10) / 290.0)
    } else if x < -145.0 {
        (10.0 / std::f64::consts::LN_10) * ((x + 290.0) / 290.0).ln()
    } else {
        (10.0 / std::f64::consts::LN_10) * (x / 290.0).ln_1p()
    }
}

fn main() {
    println!("kind,input,reference,baseline,direct,range-aware,public (relative errors)");
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split(',').collect();
        let x = f64::from_bits(u64::from_str_radix(fields[1], 16).unwrap());
        let expected: f64 = fields[2].parse().unwrap();
        let (baseline, direct, ranged, public) = if fields[0] == "nf" {
            (
                290.0 * (10.0_f64.powf(x / 10.0) - 1.0),
                290.0 * (x * (std::f64::consts::LN_10 / 10.0)).exp_m1(),
                ranged_nf(x),
                rfconversions::noise::noise_temperature_from_noise_figure(x),
            )
        } else {
            (
                10.0 * (1.0 + x / 290.0).log10(),
                (10.0 / std::f64::consts::LN_10) * (x / 290.0).ln_1p(),
                ranged_te(x),
                rfconversions::noise::noise_figure_from_noise_temperature(x),
            )
        };
        print!("{},{x:.17e},{expected:.17e}", fields[0]);
        for value in [baseline, direct, ranged, public] {
            let error = if value == expected || value.is_nan() && expected.is_nan() {
                0.0
            } else if !value.is_finite() || !expected.is_finite() || expected == 0.0 {
                f64::INFINITY
            } else {
                (value - expected).abs() / expected.abs()
            };
            print!(",{value:.17e} ({error:.3e})");
        }
        println!();
    }
}
