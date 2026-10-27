//! Practical RF engineering scenario tests.
//!
//! These integration tests verify rfconversions functions against
//! real-world RF values commonly encountered in satellite and
//! wireless system design.

use rfconversions::frequency::*;
use rfconversions::noise::*;
use rfconversions::p1db::*;
use rfconversions::power::*;

// ── Power conversions at practical levels ────────────────────────

#[test]
fn typical_transmitter_power_levels() {
    // 100W SSPA → 50 dBm → 20 dBW
    let dbm = watts_to_dbm(100.0);
    assert!((dbm - 50.0).abs() < 1e-10);
    assert!((dbm_to_dbw(dbm) - 20.0).abs() < 1e-10);

    // 1W → 30 dBm → 0 dBW
    assert!((watts_to_dbm(1.0) - 30.0).abs() < 1e-10);
    assert!((watts_to_dbw(1.0) - 0.0).abs() < 1e-10);

    // 10 mW → 10 dBm
    assert!((milliwatts_to_dbm(10.0) - 10.0).abs() < 1e-10);
}

#[test]
fn receiver_sensitivity_levels() {
    // Typical GPS receiver sensitivity: -130 dBm
    let watts = dbm_to_watts(-130.0);
    assert!((watts - 1e-16).abs() < 1e-26);

    // Roundtrip at very low power
    let roundtrip = watts_to_dbm(dbm_to_watts(-130.0));
    assert!((roundtrip - (-130.0)).abs() < 1e-10);
}

#[test]
fn dbm_dbw_roundtrip_across_range() {
    // Test roundtrip conversion across a wide dynamic range
    for dbm_val in [-174, -130, -100, -50, 0, 10, 30, 43, 50, 60] {
        let dbm = dbm_val as f64;
        let dbw = dbm_to_dbw(dbm);
        let back = dbw_to_dbm(dbw);
        assert!((back - dbm).abs() < 1e-10, "Roundtrip failed at {dbm} dBm");
    }
}

#[test]
fn watts_milliwatts_roundtrip() {
    for &mw in &[0.001, 0.01, 1.0, 100.0, 1000.0, 100_000.0] {
        let dbm = milliwatts_to_dbm(mw);
        let back = dbm_to_milliwatts(dbm);
        assert!(
            (back - mw).abs() / mw < 1e-12,
            "Roundtrip failed at {mw} mW"
        );
    }
}

// ── Noise figure cascading ──────────────────────────────────────

#[test]
fn lna_noise_temperature_from_nf() {
    // Typical LNA: NF = 0.5 dB → Te ≈ 35.4 K
    let te = noise_temperature_from_noise_figure(0.5);
    assert!((te - 35.381).abs() < 0.1);
}

#[test]
fn noise_figure_roundtrip() {
    for &nf_db in &[0.5, 1.0, 3.0, 6.0, 10.0] {
        let factor = noise_factor_from_noise_figure(nf_db);
        let temp = noise_temperature_from_noise_factor(factor);
        let factor_back = noise_factor_from_noise_temperature(temp);
        let nf_back = noise_figure_from_noise_factor(factor_back);
        assert!(
            (nf_back - nf_db).abs() < 1e-10,
            "Noise figure roundtrip failed at {nf_db} dB"
        );
    }
}

#[test]
fn three_stage_friis_paths_agree() {
    // LNA, cable loss, and mixer. The same stages can be expressed as
    // noise factor, noise figure, or equivalent noise temperature.
    let stages_db = [(0.5, 20.0), (1.0, -1.0), (8.0, -7.0)];
    let stages_factor: Vec<_> = stages_db
        .iter()
        .map(|&(nf, gain)| (noise_factor_from_noise_figure(nf), db_to_linear(gain)))
        .collect();
    let stages_temperature: Vec<_> = stages_factor
        .iter()
        .map(|&(factor, gain)| (noise_temperature_from_noise_factor(factor), gain))
        .collect();

    let factor = cascade_noise_factor(&stages_factor);
    let figure = cascade_noise_figure(&stages_db);
    let temperature = cascade_noise_temperature(&stages_temperature);

    assert!((figure - noise_figure_from_noise_factor(factor)).abs() < 1e-12);
    assert!((temperature - noise_temperature_from_noise_factor(factor)).abs() < 1e-10);
    assert!(factor > stages_factor[0].0);
}

#[test]
fn thermal_noise_floor() {
    // kTB noise power at 290K, 1 Hz bandwidth = -174 dBm/Hz
    let power_w = noise_power_from_bandwidth(290.0, 1.0);
    let power_dbm = watts_to_dbm(power_w);
    assert!(
        (power_dbm - (-174.0)).abs() < 0.1,
        "Thermal noise floor was {power_dbm} dBm, expected ~-174 dBm"
    );
    assert!((noise_density_dbm_per_hz(290.0) - power_dbm).abs() < 1e-12);
}

#[test]
fn noise_power_36mhz_transponder() {
    // 36 MHz transponder bandwidth at 290K system temp
    let bw = 36.0e6;
    let power_w = noise_power_from_bandwidth(290.0, bw);
    let power_dbm = watts_to_dbm(power_w);
    // Expected: -174 + 10*log10(36e6) ≈ -174 + 75.56 = -98.44 dBm
    assert!(
        (power_dbm - (-98.44)).abs() < 0.1,
        "36 MHz noise power was {power_dbm} dBm"
    );
}

// ── Frequency / wavelength at common bands ──────────────────────

#[test]
fn satellite_band_wavelengths() {
    // Ka-band downlink: 20 GHz → 15 mm
    let wl = frequency_to_wavelength(ghz_to_hz(20.0));
    assert!((wl * 1000.0 - 14.99).abs() < 0.01);

    // Ku-band downlink: 12 GHz → 25 mm
    let wl = frequency_to_wavelength(ghz_to_hz(12.0));
    assert!((wl * 1000.0 - 24.98).abs() < 0.02);

    // X-band: 10 GHz → 30 mm
    let wl = frequency_to_wavelength(ghz_to_hz(10.0));
    assert!((wl * 1000.0 - 29.98).abs() < 0.01);
}

#[test]
fn frequency_unit_chain_roundtrip() {
    // 27.5 GHz through all units and back
    let ghz = 27.5;
    let thz = ghz_to_thz(ghz);
    let mhz = thz_to_mhz(thz);
    let khz = mhz_to_khz(mhz);
    let hz = khz_to_hz(khz);
    let back = hz_to_ghz(hz);
    assert!(
        (back - ghz).abs() < 1e-10,
        "Frequency chain roundtrip: got {back}, expected {ghz}"
    );
}

// ── P1dB cascade scenarios ──────────────────────────────────────

#[test]
fn three_stage_receive_chain_p1db() {
    // LNA (OP1dB=15dBm, G=20dB) → Filter (OP1dB=40dBm, G=-3dB) → Mixer (OP1dB=10dBm, G=-7dB)
    // Stage 1 sets initial cumulative OP1dB
    let cum = 15.0_f64;

    // Stage 2: filter (high OP1dB, negative gain = loss)
    let cum = cascade_output_p1db(cum, 40.0, -3.0);

    // Stage 3: mixer
    let cum = cascade_output_p1db(cum, 10.0, -7.0);

    // Refer each stage limit to the final output independently: 5, 33, 10 dBm.
    // The LNA contributes the lowest output-referred limit after 10 dB loss.
    // -10*log10(10^-0.5 + 10^-3.3 + 10^-1) = 3.801463248364889 dBm.
    assert!((cum - 3.801463248364889).abs() < 1e-10);
}

#[test]
fn three_stage_linear_p1db_with_gain_and_loss() {
    // Stage output limits: 100, 50, 80 mW. Following gains: 2, then 0.5.
    let first_two = cascade_output_p1db_linear(100.0, 50.0, 2.0);
    let total = cascade_output_p1db_linear(first_two, 80.0, 0.5);

    // All limits at the final output: 100, 25, 80 mW.
    // Their reciprocals sum to 1/100 + 1/25 + 1/80 = 1/16.
    assert!((total - 16.0).abs() < 1e-12);
}

#[test]
fn p1db_input_output_roundtrip_negative_gain() {
    // Attenuator: 10 dB loss (gain = -10 dB)
    let ip1db = 20.0;
    let gain = -10.0;
    let op1db = input_to_output_db(ip1db, gain);
    let back = output_to_input_db(op1db, gain);
    assert!((back - ip1db).abs() < 1e-10);
}

// ── Friis cascade practical scenarios ────────────────────────────

#[test]
fn satellite_ground_terminal_receive_chain() {
    // Typical Ka-band ground terminal RX chain:
    // LNA (NF=0.5dB, G=25dB) → Waveguide (NF=0.3dB, G=-0.3dB) → Downconverter (NF=8dB, G=10dB)
    use rfconversions::noise::{cascade_noise_figure, noise_temperature_from_noise_figure};

    let stages_db = vec![(0.5, 25.0), (0.3, -0.3), (8.0, 10.0)];
    let nf_total = cascade_noise_figure(&stages_db);

    // LNA dominates; total should be barely above 0.5 dB
    assert!(nf_total < 0.6, "Ka-band RX NF too high: {nf_total} dB");
    assert!(nf_total > 0.5, "Cascade NF can't be less than first stage");

    // Verify system noise temperature is reasonable for Ka-band
    let tsys = noise_temperature_from_noise_figure(nf_total);
    assert!(tsys < 45.0, "System noise temp too high: {tsys} K");
}

#[test]
fn friis_order_matters_dramatically() {
    // Demonstrate why LNA goes first: same components, different order
    use rfconversions::noise::cascade_noise_figure;

    let lna = (0.5, 20.0); // NF=0.5dB, G=20dB
    let mixer = (8.0, -7.0); // NF=8dB, G=-7dB

    let good = cascade_noise_figure(&[lna, mixer]);
    let bad = cascade_noise_figure(&[mixer, lna]);

    // LNA first: ~0.58 dB, Mixer first: ~8.6 dB
    assert!(
        bad - good > 7.0,
        "Order should matter by >7 dB, got {:.1} dB difference",
        bad - good
    );
}

#[test]
fn passive_device_noise_contribution() {
    // Cable loss = 3 dB → NF = 3 dB at room temp, G = -3 dB
    // After 20 dB LNA, cable contribution is negligible
    use rfconversions::noise::cascade_noise_figure;

    let with_cable = cascade_noise_figure(&[(1.0, 20.0), (3.0, -3.0)]);
    let without = 1.0_f64; // Just the LNA

    // 3 dB cable after 20 dB gain: adds ~0.03 dB
    assert!((with_cable - without).abs() < 0.05);
}

// ── Cross-module practical calculations ─────────────────────────

#[test]
fn gt_ratio_components() {
    // G/T calculation building blocks:
    // Antenna gain = 40 dBi, System noise temp = 200K
    // G/T = 40 - 10*log10(200) = 40 - 23.01 = 16.99 dB/K
    let gain_dbi = 40.0;
    let tsys = 200.0;
    let tsys_db = linear_to_db(tsys); // 10*log10(200)
    let g_over_t = rfconversions::noise::g_over_t(gain_dbi, tsys);
    assert!((g_over_t - (gain_dbi - tsys_db)).abs() < 1e-12);
    assert!(
        (g_over_t - 16.99).abs() < 0.01,
        "G/T was {g_over_t}, expected ~16.99"
    );
}

#[test]
fn eirp_calculation() {
    // EIRP = Tx Power (dBW) + Antenna Gain (dBi)
    // 10W transmitter + 35 dBi dish
    let tx_dbw = watts_to_dbw(10.0);
    let gain_dbi = 35.0;
    let eirp = tx_dbw + gain_dbi;
    assert!(
        (eirp - 45.0).abs() < 0.01,
        "EIRP was {eirp}, expected 45.0 dBW"
    );
}
