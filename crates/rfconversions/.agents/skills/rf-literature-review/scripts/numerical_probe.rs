//! Research probe, not a production implementation or exhaustive accuracy test.
//! Compile against the checkout's library using the commands in ../SKILL.md.
//! Reference literals were computed independently with 80-digit Python Decimal;
//! see docs/literature-review.md for sources, formulas, and remaining gates.

use rfconversions::{noise, power};
use std::f64::consts::LN_10;

fn report(label: &str, current: Option<f64>, candidate: f64, reference: f64) {
    println!(
        "{label}: current={current:?}, candidate={candidate:.17e}, reference={reference:.17e}"
    );
    // A probe tolerance, not a promised cross-platform libm error bound.
    assert!(candidate.is_finite());
    assert!((candidate - reference).abs() <= reference.abs() * 2e-14);
}

// Finite inputs only: production NaN/infinity policy remains to be implemented.
fn add_power_db(a: f64, b: f64) -> f64 {
    let high = a.max(b);
    let low = a.min(b);
    high + (10.0 / LN_10) * 10.0_f64.powf((low - high) / 10.0).ln_1p()
}

fn main() {
    for (nf, reference) in [
        (1e-16, 6.677_496_769_682_733e-15),
        (0.5, 35.385_351_747_569_396),
        (3.0, 288.626_071_340_975_1),
    ] {
        report(
            &format!("NF {nf} dB -> Te K"),
            Some(noise::noise_temperature_from_noise_figure(nf)),
            290.0 * (nf * (LN_10 / 10.0)).exp_m1(),
            reference,
        );
    }
    for (te, reference) in [
        (1e-16, 1.497_567_178_976_730_4e-18),
        (35.0, 0.494_853_630_799_182_9),
        (290.0, 3.010_299_956_639_812),
    ] {
        report(
            &format!("Te {te} K -> NF dB"),
            Some(noise::noise_figure_from_noise_temperature(te)),
            (10.0 / LN_10) * (te / 290.0).ln_1p(),
            reference,
        );
    }
    for (a, b, reference) in [
        (0.0, 0.0, 3.010_299_956_639_812),
        (0.0, -160.0, 4.342_944_819_032_518e-16),
        (4000.0, 4000.0, 4_003.010_299_956_639_8),
        (-4000.0, -4000.0, -3_996.989_700_043_360_2),
    ] {
        report(
            &format!("add {a}, {b} dB (common reference)"),
            Some(power::linear_to_db(
                power::db_to_linear(a) + power::db_to_linear(b),
            )),
            add_power_db(a, b),
            reference,
        );
    }
    // Independent analytic example: (290 - 2*77)/(2 - 1) = 136 K.
    let (hot, cold, y) = (290.0, 77.0, 2.0);
    report(
        "Y-factor temperature K (no existing API)",
        None,
        (hot - cold) / (y - 1.0) - cold,
        136.0,
    );
    println!("All sampled candidates match independent references; production gates remain open.");
}
