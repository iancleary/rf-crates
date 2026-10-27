/// Integration tests matching every code example in README.md

// === Section 1: Power Conversions ===

#[test]
fn power_watts_to_dbm() {
    let dbm = rfconversions::power::watts_to_dbm(1.0);
    assert_eq!(dbm, 30.0);
}

#[test]
fn power_dbm_to_watts() {
    let watts = rfconversions::power::dbm_to_watts(30.0);
    assert_eq!(watts, 1.0);
}

#[test]
fn power_db_to_linear() {
    let linear = rfconversions::power::db_to_linear(30.0);
    assert_eq!(linear, 1000.0);
}

#[test]
fn power_linear_to_db() {
    let db = rfconversions::power::linear_to_db(1000.0);
    assert_eq!(db, 30.0);
}

// === Section 2: Frequency Conversions ===

#[test]
fn frequency_mhz_to_ghz() {
    let ghz = rfconversions::frequency::mhz_to_ghz(2400.0);
    assert_eq!(ghz, 2.4);
}

#[test]
fn frequency_ghz_to_hz() {
    let hz = rfconversions::frequency::ghz_to_hz(1.0);
    assert_eq!(hz, 1_000_000_000.0);
}

#[test]
fn frequency_khz_to_mhz() {
    let mhz = rfconversions::frequency::khz_to_mhz(1500.0);
    assert_eq!(mhz, 1.5);
}

#[test]
fn frequency_ghz_to_thz() {
    let thz = rfconversions::frequency::ghz_to_thz(1000.0);
    assert_eq!(thz, 1.0);
}

#[test]
fn frequency_to_wavelength() {
    let wavelength = rfconversions::frequency::frequency_to_wavelength(1.0e9);
    assert_eq!(wavelength, 0.299792458);
}

// === Section 3: Noise ===

#[test]
fn noise_figure_from_noise_factor() {
    let nf_db = rfconversions::noise::noise_figure_from_noise_factor(2.0);
    assert!((nf_db - 3.010299956639812).abs() < 1e-12);
}

#[test]
fn noise_factor_from_noise_figure() {
    let nf_linear = rfconversions::noise::noise_factor_from_noise_figure(3.010299956639812);
    assert_eq!(nf_linear, 2.0);
}

#[test]
fn noise_temperature_from_noise_factor() {
    let temp = rfconversions::noise::noise_temperature_from_noise_factor(2.0);
    assert_eq!(temp, 290.0);
}

#[test]
fn noise_factor_from_noise_temperature() {
    let factor = rfconversions::noise::noise_factor_from_noise_temperature(290.0);
    assert_eq!(factor, 2.0);
}

#[test]
fn noise_temperature_from_noise_figure() {
    let temp = rfconversions::noise::noise_temperature_from_noise_figure(6.0);
    assert!((temp - 864.510794605142).abs() < 1e-6);
}

#[test]
fn noise_figure_from_noise_temperature() {
    let nf_db = rfconversions::noise::noise_figure_from_noise_temperature(290.0);
    assert!((nf_db - 3.010299956639812).abs() < 1e-12);
}

#[test]
fn noise_power_from_bandwidth() {
    let noise_power = rfconversions::noise::noise_power_from_bandwidth(290.0, 100.0e6);
    // Verify it's kTB using exact Boltzmann constant
    let expected = rfconversions::constants::BOLTZMANN * 290.0 * 100.0e6;
    assert_eq!(noise_power, expected);
}

// === Section 4: P1dB ===

#[test]
fn p1db_input_to_output() {
    let output_p1db = rfconversions::p1db::input_to_output_db(5.0, 30.0);
    assert_eq!(output_p1db, 34.0);
}

#[test]
fn p1db_output_to_input() {
    let input_p1db = rfconversions::p1db::output_to_input_db(34.0, 30.0);
    assert_eq!(input_p1db, 5.0);
}

#[test]
fn p1db_cascade() {
    let output_mw = rfconversions::p1db::cascade_output_p1db_linear(100.0, 50.0, 2.0);
    assert!((output_mw - 40.0).abs() < 1e-12);

    let output_dbm = rfconversions::p1db::cascade_output_p1db(34.0, 20.0, 30.0);
    assert!((output_dbm - 19.999827107694083).abs() < 1e-10);
}

// === Section 5: Constants ===

#[test]
fn speed_of_light() {
    let c = rfconversions::constants::SPEED_OF_LIGHT;
    assert_eq!(c, 299_792_458.0);
}
