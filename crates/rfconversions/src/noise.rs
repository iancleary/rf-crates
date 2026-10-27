/// Convert noise factor (linear) to noise temperature (Kelvin).
///
/// Uses T₀ = 290 K reference temperature.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_temperature_from_noise_factor;
/// assert_eq!(noise_temperature_from_noise_factor(2.0), 290.0);
/// ```
#[doc(alias = "F")]
#[doc(alias = "Te")]
#[must_use]
pub fn noise_temperature_from_noise_factor(noise_factor: f64) -> f64 {
    crate::constants::T0 * (noise_factor - 1.0)
}

/// Convert noise figure (dB) to noise temperature (Kelvin).
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_temperature_from_noise_figure;
/// let temp = noise_temperature_from_noise_figure(3.0);
/// assert!((temp - 288.626).abs() < 0.001);
/// ```
#[doc(alias = "NF")]
#[doc(alias = "Te")]
#[must_use]
pub fn noise_temperature_from_noise_figure(noise_figure: f64) -> f64 {
    let noise_factor: f64 = noise_factor_from_noise_figure(noise_figure);
    noise_temperature_from_noise_factor(noise_factor)
}

/// Convert noise figure (dB) to noise factor (linear).
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_factor_from_noise_figure;
/// assert_eq!(noise_factor_from_noise_figure(3.010299956639812), 2.0);
/// ```
#[doc(alias = "NF")]
#[doc(alias = "F")]
#[must_use]
pub fn noise_factor_from_noise_figure(noise_figure: f64) -> f64 {
    crate::power::db_to_linear(noise_figure)
}

/// Convert noise temperature (Kelvin) to noise factor (linear).
///
/// Uses T₀ = 290 K reference temperature.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_factor_from_noise_temperature;
/// assert_eq!(noise_factor_from_noise_temperature(290.0), 2.0);
/// ```
#[doc(alias = "Te")]
#[doc(alias = "F")]
#[must_use]
pub fn noise_factor_from_noise_temperature(noise_temperature: f64) -> f64 {
    1.0 + (noise_temperature / crate::constants::T0)
}

/// Convert noise temperature (Kelvin) to noise figure (dB).
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_figure_from_noise_temperature;
/// let nf = noise_figure_from_noise_temperature(290.0);
/// assert!((nf - 3.0103).abs() < 0.001);
/// ```
#[doc(alias = "Te")]
#[doc(alias = "NF")]
#[must_use]
pub fn noise_figure_from_noise_temperature(noise_temperature: f64) -> f64 {
    let noise_factor: f64 = noise_factor_from_noise_temperature(noise_temperature);
    noise_figure_from_noise_factor(noise_factor)
}

/// Convert noise factor (linear) to noise figure (dB).
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_figure_from_noise_factor;
/// let nf = noise_figure_from_noise_factor(2.0);
/// assert!((nf - 3.0103).abs() < 0.001);
/// ```
#[doc(alias = "NF")]
#[doc(alias = "F")]
#[must_use]
pub fn noise_figure_from_noise_factor(noise_factor: f64) -> f64 {
    crate::power::linear_to_db(noise_factor)
}

/// Calculate thermal noise power (watts) from temperature (kelvin) and bandwidth (hertz).
///
/// Uses Boltzmann's constant k = 1.380649 × 10⁻²³ J/K.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_power_from_bandwidth;
/// let power = noise_power_from_bandwidth(290.0, 1.0);
/// assert!((power - 4.004e-21).abs() < 1e-24);
/// ```
#[doc(alias = "kTB")]
#[doc(alias = "thermal noise")]
#[must_use]
pub fn noise_power_from_bandwidth(temperature: f64, bandwidth: f64) -> f64 {
    crate::constants::BOLTZMANN * temperature * bandwidth
}

/// Cascade noise factor using the Friis formula.
///
/// Computes the total noise factor of a chain of stages using:
///
/// F_total = F₁ + (F₂ - 1)/G₁ + (F₃ - 1)/(G₁·G₂) + ...
///
/// Each stage is represented as `(noise_factor, gain)` where both values
/// are in **linear** (not dB) units.
///
/// # Arguments
///
/// * `stages` - Slice of `(noise_factor, gain)` tuples in linear units.
///
/// # Panics
///
/// Panics if `stages` is empty.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::cascade_noise_factor;
///
/// // LNA (NF=0.5dB, G=20dB) → Cable (NF=1dB, G=-1dB) → Mixer (NF=8dB, G=-7dB)
/// let stages = vec![
///     (1.1220, 100.0),   // LNA: F=1.122, G=100
///     (1.2589, 0.7943),  // Cable: F=1.259, G=0.794
///     (6.3096, 0.1995),  // Mixer: F=6.310, G=0.200
/// ];
/// let f_total = cascade_noise_factor(&stages);
/// // LNA dominates — total NF barely above LNA's
/// assert!((f_total - 1.1914).abs() < 0.001);
/// ```
#[doc(alias = "Friis")]
#[doc(alias = "F")]
#[must_use]
pub fn cascade_noise_factor(stages: &[(f64, f64)]) -> f64 {
    cascade_noise_terms(stages, 1.0)
}

/// Cascade noise figure (dB) using the Friis formula.
///
/// Convenience wrapper around [`cascade_noise_factor`] that accepts
/// and returns values in dB.
///
/// Each stage is `(noise_figure_db, gain_db)`.
///
/// # Panics
///
/// Panics if `stages` is empty.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::cascade_noise_figure;
///
/// // LNA (NF=0.5dB, G=20dB) → Cable (NF=1dB, G=-1dB) → Mixer (NF=8dB, G=-7dB)
/// let stages = vec![(0.5, 20.0), (1.0, -1.0), (8.0, -7.0)];
/// let nf_total = cascade_noise_figure(&stages);
/// assert!((nf_total - 0.754).abs() < 0.01);
/// ```
#[doc(alias = "Friis")]
#[doc(alias = "NF")]
#[must_use]
pub fn cascade_noise_figure(stages: &[(f64, f64)]) -> f64 {
    let linear_stages: Vec<(f64, f64)> = stages
        .iter()
        .map(|&(nf_db, gain_db)| {
            (
                noise_factor_from_noise_figure(nf_db),
                crate::power::db_to_linear(gain_db),
            )
        })
        .collect();

    noise_figure_from_noise_factor(cascade_noise_factor(&linear_stages))
}

/// Cascade noise temperature using the Friis formula.
///
/// Computes the total equivalent noise temperature of a chain of stages:
///
/// T_total = T₁ + T₂/G₁ + T₃/(G₁·G₂) + ...
///
/// Each stage is `(noise_temperature_kelvin, gain_linear)`.
///
/// # Panics
///
/// Panics if `stages` is empty.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::cascade_noise_temperature;
///
/// // LNA (Te=35K, G=100) → Cable (Te=75K, G=0.794)
/// let stages = vec![(35.0, 100.0), (75.0, 0.794)];
/// let t_total = cascade_noise_temperature(&stages);
/// assert!((t_total - 35.75).abs() < 0.01);
/// ```
#[doc(alias = "Friis")]
#[doc(alias = "Te")]
#[must_use]
pub fn cascade_noise_temperature(stages: &[(f64, f64)]) -> f64 {
    cascade_noise_terms(stages, 0.0)
}

// Friis uses the same preceding gain for factor and temperature. Factor
// contributions are F - 1; temperature contributions are T.
fn cascade_noise_terms(stages: &[(f64, f64)], reference: f64) -> f64 {
    assert!(!stages.is_empty(), "stages must not be empty");

    let mut total = stages[0].0;
    let mut cumulative_gain = stages[0].1;

    for &(value, gain) in &stages[1..] {
        total += (value - reference) / cumulative_gain;
        cumulative_gain *= gain;
    }

    total
}

/// Calculate system figure of merit G/T in dB/K.
///
/// G/T is the ratio of antenna gain (dBi) to system noise temperature (K),
/// a key metric for receive system sensitivity.
///
/// # Arguments
///
/// * `antenna_gain_dbi` - Antenna gain in dBi
/// * `system_noise_temperature` - System noise temperature in kelvin
///
/// # Examples
///
/// ```
/// use rfconversions::noise::g_over_t;
/// // 40 dBi antenna, 200 K system noise temperature
/// let got = g_over_t(40.0, 200.0);
/// assert!((got - 16.99).abs() < 0.01);
/// ```
#[doc(alias = "G/T")]
#[doc(alias = "figure of merit")]
#[must_use]
pub fn g_over_t(antenna_gain_dbi: f64, system_noise_temperature: f64) -> f64 {
    antenna_gain_dbi - crate::power::linear_to_db(system_noise_temperature)
}

/// Calculate noise power spectral density N₀ in dBm/Hz.
///
/// N₀ = 10·log₁₀(k·T) + 30, where the +30 converts from dBW to dBm.
///
/// At 290 K, this gives the well-known -174 dBm/Hz thermal noise floor.
///
/// # Examples
///
/// ```
/// use rfconversions::noise::noise_density_dbm_per_hz;
/// let n0 = noise_density_dbm_per_hz(290.0);
/// assert!((n0 - (-174.0)).abs() < 0.1);
/// ```
#[doc(alias = "N0")]
#[doc(alias = "noise density")]
#[must_use]
pub fn noise_density_dbm_per_hz(temperature: f64) -> f64 {
    crate::power::watts_to_dbm(crate::constants::BOLTZMANN * temperature)
}

// Noise Figure of Passive Device
// https://www.microwaves101.com/encyclopedias/noise-temperature
// "Linear passive devices have noise figure equal to their loss. Expressed in dB, the NF is equal to -S21(dB). Something with one dB loss has one dB noise figure.
// May I suggest a more refined definition of this rule? This statement is true only if the passive linear device is at room temperature. However, if it is at a different physical temperature than room temperature (or To for that matter), the noise figure will be different. If I did my calculation properly, I believe that the noise figure would be
// F = 1+(1/G-1)*Tp/To
// Where G is the gain of the device (less than or equal to 1), and Tp is the physical temperature of the device. Therefore, I would recommend that the statement should say, "Linear passive devices at room temperature have a noise figure equal to their loss. Expressed in dB, the NF is equal to -S21(dB). Something with one dB loss has one dB noise figure at room temperature." I know that the NF wouldn't change very much if the device is at a physical temperature near room temperature, but if some poor slob is working at temperatures very different than room temperature, their assumption that the NF would be equal to the loss would be incorrect.
// I hope that this helps."

#[cfg(test)]
mod tests {

    #[test]
    fn noise_temperature_from_noise_factor() {
        let noise_factor: f64 = 2.0;

        let noise_temperature: f64 = super::noise_temperature_from_noise_factor(noise_factor);

        assert_eq!(290.0, noise_temperature);
    }

    #[test]
    fn another_noise_temperature_from_noise_factor() {
        let noise_factor: f64 = 4.0;

        let noise_temperature: f64 = super::noise_temperature_from_noise_factor(noise_factor);

        assert_eq!(870.0, noise_temperature);
    }

    #[test]
    fn noise_temperature_from_noise_figure() {
        let noise_figure: f64 = 3.0;

        let noise_temperature: f64 = super::noise_temperature_from_noise_figure(noise_figure);

        assert_eq!(288.62607134097505, noise_temperature);
    }

    #[test]
    fn another_noise_temperature_from_noise_figure() {
        let noise_figure: f64 = 6.0;

        let noise_temperature: f64 = super::noise_temperature_from_noise_figure(noise_figure);

        assert_eq!(864.510794605142, noise_temperature);
    }

    #[test]
    fn noise_factor_from_noise_temperature() {
        let noise_temperature: f64 = 290.0;

        let noise_factor: f64 = super::noise_factor_from_noise_temperature(noise_temperature);

        assert_eq!(2.0, noise_factor);
    }

    #[test]
    fn another_noise_factor_from_noise_temperature() {
        let noise_temperature: f64 = 290.0;

        let noise_factor: f64 = super::noise_factor_from_noise_temperature(noise_temperature);

        assert_eq!(2.0, noise_factor);
    }

    #[test]
    fn noise_factor_from_noise_figure() {
        let noise_figure: f64 = 3.010299956639812;

        let noise_factor: f64 = super::noise_factor_from_noise_figure(noise_figure);

        assert_eq!(2.0, noise_factor);
    }

    #[test]
    fn another_noise_factor_from_noise_figure() {
        let noise_figure: f64 = 6.020599913279624;

        let noise_factor: f64 = super::noise_factor_from_noise_figure(noise_figure);

        assert_eq!(4.0, noise_factor);
    }

    #[test]
    fn noise_figure_from_noise_temperature() {
        let noise_temperature: f64 = 864.510794605142;

        let noise_figure: f64 = super::noise_figure_from_noise_temperature(noise_temperature);

        assert_eq!(6.0, noise_figure);
    }

    #[test]
    fn another_noise_figure_from_noise_temperature() {
        let noise_temperature: f64 = 290.0;

        let noise_figure: f64 = super::noise_figure_from_noise_temperature(noise_temperature);

        assert_eq!(3.010299956639812, noise_figure);
    }

    #[test]
    fn noise_figure_from_noise_factor() {
        let noise_factor: f64 = 2.0;

        let noise_figure: f64 = super::noise_figure_from_noise_factor(noise_factor);

        assert_eq!(3.010299956639812, noise_figure);
    }

    #[test]
    fn another_noise_figure_from_noise_factor() {
        let noise_factor: f64 = 4.0;

        let noise_figure: f64 = super::noise_figure_from_noise_factor(noise_factor);

        assert_eq!(6.020599913279624, noise_figure);
    }

    #[test]
    fn noise_power_from_bandwidth() {
        let bandwidth: f64 = 100.0e6;
        let temperature: f64 = 290.0;

        let noise_power: f64 = super::noise_power_from_bandwidth(temperature, bandwidth);

        let noise_power_dbm: f64 = 10.0 * (noise_power.log10() + 3.0);

        // kTB at 290K, 100 MHz: ~-94 dBm
        assert!((noise_power_dbm - (-93.975)).abs() < 0.01);
    }

    #[test]
    fn noise_factor_one_gives_zero_temperature() {
        let noise_temperature: f64 = super::noise_temperature_from_noise_factor(1.0);
        assert_eq!(0.0, noise_temperature);
    }

    #[test]
    fn roundtrip_noise_figure_temperature_noise_figure() {
        let original_nf: f64 = 3.0;
        let temperature: f64 = super::noise_temperature_from_noise_figure(original_nf);
        let result_nf: f64 = super::noise_figure_from_noise_temperature(temperature);
        assert!((original_nf - result_nf).abs() < 1e-10);
    }

    // ── Friis cascade tests ──────────────────────────────────────

    #[test]
    fn cascade_noise_factor_single_stage() {
        let f = super::cascade_noise_factor(&[(2.0, 10.0)]);
        assert_eq!(f, 2.0);
    }

    #[test]
    fn cascade_noise_factor_two_stage_lna_dominant() {
        // LNA: F=1.12 (0.5dB), G=100 (20dB)
        // Mixer: F=6.31 (8dB), G=0.2 (-7dB)
        // F_total = 1.12 + (6.31 - 1)/100 = 1.12 + 0.0531 = 1.1731
        let f = super::cascade_noise_factor(&[(1.12, 100.0), (6.31, 0.2)]);
        assert!((f - 1.1731).abs() < 0.001);
    }

    #[test]
    fn cascade_noise_factor_bad_order() {
        // Mixer first, then LNA — much worse
        let good = super::cascade_noise_factor(&[(1.12, 100.0), (6.31, 0.2)]);
        let bad = super::cascade_noise_factor(&[(6.31, 0.2), (1.12, 100.0)]);
        assert!(bad > good, "Putting mixer first should give worse NF");
        // bad = 6.31 + (1.12-1)/0.2 = 6.31 + 0.6 = 6.91
        assert!((bad - 6.91).abs() < 0.01);
    }

    #[test]
    #[should_panic(expected = "stages must not be empty")]
    fn cascade_noise_factor_empty_panics() {
        let _ = super::cascade_noise_factor(&[]);
    }

    #[test]
    fn cascade_noise_figure_three_stage_rx() {
        // LNA (0.5dB, 20dB) → Cable (1dB, -1dB) → Mixer (8dB, -7dB)
        let nf = super::cascade_noise_figure(&[(0.5, 20.0), (1.0, -1.0), (8.0, -7.0)]);
        // Should be barely above LNA NF since LNA gain dominates
        assert!(nf < 1.0, "Cascade NF should be < 1 dB, got {nf}");
        assert!(nf > 0.5, "Cascade NF should be > LNA NF, got {nf}");
    }

    #[test]
    #[should_panic(expected = "stages must not be empty")]
    fn cascade_noise_figure_empty_panics() {
        let _ = super::cascade_noise_figure(&[]);
    }

    #[test]
    fn cascade_noise_temperature_two_stage() {
        // LNA: Te=35K, G=100 → Cable: Te=75K, G=0.794
        let t = super::cascade_noise_temperature(&[(35.0, 100.0), (75.0, 0.794)]);
        // T_total = 35 + 75/100 = 35.75
        assert!((t - 35.75).abs() < 0.01);
    }

    #[test]
    fn cascade_noise_temperature_matches_factor() {
        // Verify consistency: cascade via temperature should match cascade via factor
        let nf1 = 0.5_f64;
        let g1 = 20.0_f64;
        let nf2 = 8.0_f64;
        let g2 = -7.0_f64;

        let nf_cascade = super::cascade_noise_figure(&[(nf1, g1), (nf2, g2)]);

        let t1 = super::noise_temperature_from_noise_figure(nf1);
        let t2 = super::noise_temperature_from_noise_figure(nf2);
        let g1_lin = crate::power::db_to_linear(g1);
        let g2_lin = crate::power::db_to_linear(g2);
        let t_cascade = super::cascade_noise_temperature(&[(t1, g1_lin), (t2, g2_lin)]);
        let nf_from_temp = super::noise_figure_from_noise_temperature(t_cascade);

        assert!(
            (nf_cascade - nf_from_temp).abs() < 1e-10,
            "NF methods disagree: {nf_cascade} vs {nf_from_temp}"
        );
    }

    #[test]
    #[should_panic(expected = "stages must not be empty")]
    fn cascade_noise_temperature_empty_panics() {
        let _ = super::cascade_noise_temperature(&[]);
    }

    #[test]
    fn noise_power_from_bandwidth_known_ktb() {
        // kTB at 290K, 1 Hz bandwidth
        let noise_power: f64 = super::noise_power_from_bandwidth(290.0, 1.0);
        let expected: f64 = crate::constants::BOLTZMANN * 290.0;
        assert_eq!(expected, noise_power);
    }

    #[test]
    fn g_over_t_typical_earth_station() {
        // 40 dBi dish, 200 K system noise
        let got = super::g_over_t(40.0, 200.0);
        // G/T = 40 - 10*log10(200) = 40 - 23.01 = 16.99
        assert!((got - 16.99).abs() < 0.01);
    }

    #[test]
    fn noise_density_at_290k() {
        let n0 = super::noise_density_dbm_per_hz(290.0);
        // Standard thermal noise floor: -174 dBm/Hz
        assert!((n0 - (-174.0)).abs() < 0.1);
    }

    #[test]
    fn noise_density_at_zero_kelvin_is_neg_infinity() {
        let n0 = super::noise_density_dbm_per_hz(0.0);
        assert!(n0.is_infinite() && n0 < 0.0);
    }
}
