/// Convert input P1dB to output P1dB with the same power reference (dBm or dBW).
///
/// `gain_db` is the small-signal power gain in dB. At the compression point,
/// the gain is 1 dB lower: `output_p1db = input_p1db + gain_db - 1`.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::input_to_output_db;
/// assert_eq!(input_to_output_db(5.0, 30.0), 34.0);
/// ```
#[doc(alias = "IP1dB")]
#[doc(alias = "OP1dB")]
#[must_use]
pub fn input_to_output_db(input_p1db: f64, gain_db: f64) -> f64 {
    input_p1db + (gain_db - 1.0)
}

/// Convert output P1dB to input P1dB with the same power reference (dBm or dBW).
///
/// `gain_db` is the small-signal power gain in dB. At the compression point,
/// the gain is 1 dB lower: `input_p1db = output_p1db - gain_db + 1`.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::output_to_input_db;
/// assert_eq!(output_to_input_db(34.0, 30.0), 5.0);
/// ```
#[doc(alias = "IP1dB")]
#[doc(alias = "OP1dB")]
#[must_use]
pub fn output_to_input_db(output_p1db: f64, gain_db: f64) -> f64 {
    output_p1db - (gain_db - 1.0)
}

/// Estimate output P1dB after appending a stage to a cascade (linear powers).
///
/// Both P1dB inputs must use the same linear power unit, such as watts or
/// milliwatts. The result uses that unit. `current_stage_gain_linear` is the
/// dimensionless small-signal power gain of the appended stage.
///
/// Refer the preceding cascade's output limit to the new output plane:
/// `P_total = 1 / (1 / (P_previous * G_current) + 1 / P_current)`.
/// Each input limit is specified at its own stage or cascade output.
///
/// This is an engineering estimate for a scalar cascade whose stages operate
/// in their linear region before compression. It combines reciprocal power
/// limits instead of modeling nonlinear transfer curves. Exact compression
/// requires each device's input/output transfer curve.
///
/// Reference: <https://www.rfcafe.com/references/electrical/p1db.htm>
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::cascade_output_p1db_linear;
/// // Limits: 100 mW at the preceding output, 50 mW at the new stage output.
/// // Gain 2 refers the preceding limit to 200 mW at the new output.
/// // 1 / (1/200 + 1/50) = 40 mW.
/// let result = cascade_output_p1db_linear(100.0, 50.0, 2.0);
/// assert!((result - 40.0).abs() < 1e-12);
/// ```
#[doc(alias = "OP1dB")]
#[must_use]
pub fn cascade_output_p1db_linear(
    cumulative_output_p1db_linear: f64,
    current_stage_output_p1db_linear: f64,
    current_stage_gain_linear: f64,
) -> f64 {
    1.0 / ((1.0 / (cumulative_output_p1db_linear * current_stage_gain_linear))
        + (1.0 / current_stage_output_p1db_linear))
}

/// Estimate output P1dB after appending a stage to a cascade (dBm or dBW).
///
/// Both P1dB inputs must use the same logarithmic power reference (dBm or dBW).
/// The result uses that reference. `current_stage_gain` is the appended stage's
/// small-signal power gain in dB. Each limit is specified at its own output.
///
/// Converts to linear powers, applies [`cascade_output_p1db_linear`], then
/// converts back. The same engineering approximation and model limits apply.
///
/// # Examples
///
/// ```
/// use rfconversions::p1db::cascade_output_p1db;
/// // 34 dBm preceding limit, 20 dBm new stage limit, 30 dB new stage gain.
/// let result = cascade_output_p1db(34.0, 20.0, 30.0);
/// assert!((result - 19.999827107694083).abs() < 1e-10);
/// ```
#[doc(alias = "OP1dB")]
#[doc(alias = "P1dB")]
#[must_use]
pub fn cascade_output_p1db(
    cumulative_output_p1db: f64,
    current_stage_output_p1db: f64,
    current_stage_gain: f64,
) -> f64 {
    let cumulative_output_p1db_linear = crate::power::db_to_linear(cumulative_output_p1db);
    let current_stage_output_linear = crate::power::db_to_linear(current_stage_output_p1db);
    let current_stage_gain_linear = crate::power::db_to_linear(current_stage_gain);
    let cascade_output_p1db_linear = cascade_output_p1db_linear(
        cumulative_output_p1db_linear,
        current_stage_output_linear,
        current_stage_gain_linear,
    );
    crate::power::linear_to_db(cascade_output_p1db_linear)
}

#[cfg(test)]
mod tests {

    #[test]
    fn input_to_output_p1db() {
        let input_p1db: f64 = 5.0;

        let gain_db: f64 = 30.0;

        let output_p1db = crate::p1db::input_to_output_db(input_p1db, gain_db);
        assert_eq!(output_p1db, 34.0);
    }

    #[test]
    fn output_to_input_p1db() {
        let output_p1db: f64 = 34.0;

        let gain_db: f64 = 30.0;

        let input_p1db = crate::p1db::output_to_input_db(output_p1db, gain_db);
        assert_eq!(input_p1db, 5.0);
    }

    // https://www.rfcafe.com/references/electrical/p1db.htm
    #[test]
    fn cascade_output_p1db() {
        let cumulative_output_p1db: f64 = 34.0;

        let current_stage_output_p1db: f64 = 20.0;

        let current_stage_gain: f64 = 30.0;

        let cascade_output_p1db = crate::p1db::cascade_output_p1db(
            cumulative_output_p1db,
            current_stage_output_p1db,
            current_stage_gain,
        );
        // Output-referred limits are 64 dBm and 20 dBm. An independent
        // decimal calculation of their reciprocal sum gives this estimate.
        assert!((cascade_output_p1db - 19.999827107694083).abs() < 1e-10);
    }

    #[test]
    fn roundtrip_input_to_output_to_input() {
        let input_p1db: f64 = -10.0;
        let gain_db: f64 = 25.0;

        let output_p1db = crate::p1db::input_to_output_db(input_p1db, gain_db);
        let result = crate::p1db::output_to_input_db(output_p1db, gain_db);

        assert_eq!(input_p1db, result);
    }

    #[test]
    fn input_to_output_negative_gain_attenuator() {
        // Attenuator: gain = -10 dB, input P1dB = 20 dBm
        // Output P1dB = 20 + (-10 - 1) = 9 dBm
        let result = crate::p1db::input_to_output_db(20.0, -10.0);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn input_to_output_zero_gain() {
        // Unity gain device: output P1dB = input P1dB - 1
        let result = crate::p1db::input_to_output_db(10.0, 0.0);
        assert_eq!(result, 9.0);
    }

    #[test]
    fn output_to_input_negative_gain() {
        // Attenuator: output P1dB = 9 dBm, gain = -10 dB
        // Input P1dB = 9 - (-10 - 1) = 9 + 11 = 20
        let result = crate::p1db::output_to_input_db(9.0, -10.0);
        assert_eq!(result, 20.0);
    }

    #[test]
    fn cascade_three_stage_amplifier_chain() {
        // Three-stage cascade: apply formula iteratively
        // Stage 1: OP1dB = 30 dBm, Gain = 20 dB
        // Stage 2: OP1dB = 25 dBm, Gain = 15 dB
        // Stage 3: OP1dB = 35 dBm, Gain = 10 dB
        let stages: Vec<(f64, f64)> = vec![(30.0, 20.0), (25.0, 15.0), (35.0, 10.0)];

        let mut cumulative = stages[0].0; // first stage OP1dB
        for &(op1db, gain) in &stages[1..] {
            cumulative = crate::p1db::cascade_output_p1db(cumulative, op1db, gain);
        }

        // Refer all three limits to the final output: 55, 35, and 35 dBm.
        // -10*log10(10^-5.5 + 10^-3.5 + 10^-3.5) = 31.96803942579511 dBm.
        assert!((cumulative - 31.96803942579511).abs() < 1e-10);
    }

    #[test]
    fn cascade_identical_stages() {
        // Two identical stages: OP1dB = 20 dBm, Gain = 10 dB each
        let result = crate::p1db::cascade_output_p1db(20.0, 20.0, 10.0);
        // Output-referred limits are 1000 and 100 mW: 1000/11 mW total.
        assert!((result - 19.58607314841775).abs() < 1e-10);
    }

    #[test]
    fn cascade_linear_known_value() {
        // Simple case: both stages OP1dB = 100 mW linear, gain = 1 (0 dB)
        // Output-referred limits are both 100 mW: 1 / (1/100 + 1/100) = 50.
        let result = crate::p1db::cascade_output_p1db_linear(100.0, 100.0, 1.0);
        assert_eq!(result, 50.0);
    }

    #[test]
    fn cascade_high_gain_refers_previous_limit_upward() {
        // High gain refers the first stage's limit above the second stage's.
        // cumulative OP1dB = 20 dBm, next stage OP1dB = 40 dBm, gain = 40 dB
        let result = crate::p1db::cascade_output_p1db(20.0, 40.0, 40.0);
        // Output-referred limits are 60 and 40 dBm: 1e6/101 mW total.
        assert!((result - 39.95678626217357).abs() < 1e-10);
    }

    #[test]
    fn cascade_linear_with_gain() {
        // The first limit becomes 200 mW at the second output.
        // 1 / (1/200 + 1/50) = 40 mW.
        let result = crate::p1db::cascade_output_p1db_linear(100.0, 50.0, 2.0);
        assert!((result - 40.0).abs() < 1e-12);
    }

    #[test]
    fn cascade_linear_with_loss() {
        // The first limit becomes 50 mW at the second output.
        // 1 / (1/50 + 1/50) = 25 mW.
        let result = crate::p1db::cascade_output_p1db_linear(100.0, 50.0, 0.5);
        assert!((result - 25.0).abs() < 1e-12);
    }

    #[test]
    fn cascade_db_with_loss() {
        // OP1dB = 20 dBm for each stage; the second stage loses 10 dB.
        // Output-referred limits are 10 and 100 mW: 100/11 mW total.
        let result = crate::p1db::cascade_output_p1db(20.0, 20.0, -10.0);
        assert!((result - 9.58607314841775).abs() < 1e-10);
    }

    #[test]
    fn cascade_db_with_unity_gain() {
        // Two 20 dBm limits at the same output plane combine to 50 mW.
        let result = crate::p1db::cascade_output_p1db(20.0, 20.0, 0.0);
        assert!((result - 16.989700043360187).abs() < 1e-10);
    }

    #[test]
    fn cascade_preserves_power_reference() {
        // The same physical limits in watts and dBW instead of mW and dBm.
        let watts = crate::p1db::cascade_output_p1db_linear(0.1, 0.05, 2.0);
        assert!((watts - 0.04).abs() < 1e-14);
        let dbw = crate::p1db::cascade_output_p1db(4.0, -10.0, 30.0);
        assert!((dbw - (-10.000172892305917)).abs() < 1e-10);
    }

    #[test]
    fn roundtrip_multiple_gains() {
        // Roundtrip for several gain values
        for gain in [-20.0, -5.0, 0.0, 10.0, 30.0, 50.0] {
            let ip1db = 5.0;
            let op1db = crate::p1db::input_to_output_db(ip1db, gain);
            let back = crate::p1db::output_to_input_db(op1db, gain);
            assert!(
                (back - ip1db).abs() < 1e-10,
                "roundtrip failed for gain={gain}"
            );
        }
    }
}
