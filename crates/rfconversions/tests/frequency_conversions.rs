use rfconversions::frequency::*;

const EPSILON: f64 = 1e-6;

// ─── Wavelength ↔ Frequency round-trips ───

#[test]
fn wavelength_roundtrip_1ghz() {
    let freq = 1.0e9;
    let result = wavelength_to_frequency(frequency_to_wavelength(freq));
    assert!((result - freq).abs() < 1.0, "got {result}");
}

#[test]
fn wavelength_roundtrip_28ghz() {
    let freq = 28.0e9; // 5G mmWave
    let wl = frequency_to_wavelength(freq);
    assert!((wl - 0.010706873).abs() < 1e-6);
    let back = wavelength_to_frequency(wl);
    assert!((back - freq).abs() < 1.0);
}

#[test]
fn wavelength_roundtrip_77ghz() {
    let freq = 77.0e9; // automotive radar
    let wl = frequency_to_wavelength(freq);
    assert!(wl > 0.003 && wl < 0.004, "got {wl}");
}

// ─── THz conversions ───

#[test]
fn thz_to_all_units() {
    let thz = 1.0;
    assert!((thz_to_ghz(thz) - 1000.0).abs() < EPSILON);
    assert!((thz_to_mhz(thz) - 1_000_000.0).abs() < EPSILON);
    assert!((thz_to_khz(thz) - 1_000_000_000.0).abs() < 1.0);
    assert!((thz_to_hz(thz) - 1_000_000_000_000.0).abs() < 1000.0);
}

#[test]
fn thz_roundtrip_via_ghz() {
    let original = 0.345;
    let result = ghz_to_thz(thz_to_ghz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn thz_roundtrip_via_mhz() {
    let original = 2.4;
    let result = mhz_to_thz(thz_to_mhz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn thz_roundtrip_via_khz() {
    let original = 0.1;
    let result = khz_to_thz(thz_to_khz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn thz_roundtrip_via_hz() {
    let original = 0.5;
    let result = hz_to_thz(thz_to_hz(original));
    assert!((result - original).abs() < EPSILON);
}

// ─── GHz conversions ───

#[test]
fn ghz_to_all_units() {
    let ghz = 12.0; // Ku-band
    assert!((ghz_to_thz(ghz) - 0.012).abs() < EPSILON);
    assert!((ghz_to_mhz(ghz) - 12_000.0).abs() < EPSILON);
    assert!((ghz_to_khz(ghz) - 12_000_000.0).abs() < 1.0);
    assert!((ghz_to_hz(ghz) - 12_000_000_000.0).abs() < 1000.0);
}

#[test]
fn ghz_roundtrip_via_mhz() {
    let original = 20.0; // Ka-band
    let result = mhz_to_ghz(ghz_to_mhz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn ghz_roundtrip_via_khz() {
    let original = 5.8; // ISM band
    let result = khz_to_ghz(ghz_to_khz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn ghz_roundtrip_via_hz() {
    let original = 2.4; // WiFi
    let result = hz_to_ghz(ghz_to_hz(original));
    assert!((result - original).abs() < EPSILON);
}

// ─── MHz conversions ───

#[test]
fn mhz_to_all_units() {
    let mhz = 2400.0; // WiFi
    assert!((mhz_to_thz(mhz) - 0.0024).abs() < EPSILON);
    assert!((mhz_to_ghz(mhz) - 2.4).abs() < EPSILON);
    assert!((mhz_to_khz(mhz) - 2_400_000.0).abs() < 1.0);
    assert!((mhz_to_hz(mhz) - 2_400_000_000.0).abs() < 1000.0);
}

#[test]
fn mhz_roundtrip_via_khz() {
    let original = 137.0; // NOAA weather sat
    let result = khz_to_mhz(mhz_to_khz(original));
    assert!((result - original).abs() < EPSILON);
}

#[test]
fn mhz_roundtrip_via_hz() {
    let original = 915.0; // ISM band
    let result = hz_to_mhz(mhz_to_hz(original));
    assert!((result - original).abs() < EPSILON);
}

// ─── kHz conversions ───

#[test]
fn khz_to_all_units() {
    let khz = 500_000.0; // 500 MHz
    assert!((khz_to_thz(khz) - 0.0005).abs() < EPSILON);
    assert!((khz_to_ghz(khz) - 0.5).abs() < EPSILON);
    assert!((khz_to_mhz(khz) - 500.0).abs() < EPSILON);
    assert!((khz_to_hz(khz) - 500_000_000.0).abs() < 1000.0);
}

#[test]
fn khz_roundtrip_via_hz() {
    let original = 10_700_000.0; // 10.7 GHz IF
    let result = hz_to_khz(khz_to_hz(original));
    assert!((result - original).abs() < EPSILON);
}

// ─── Hz conversions ───

#[test]
fn hz_to_all_units() {
    let hz = 10_000_000_000.0; // 10 GHz X-band
    assert!((hz_to_thz(hz) - 0.01).abs() < EPSILON);
    assert!((hz_to_ghz(hz) - 10.0).abs() < EPSILON);
    assert!((hz_to_mhz(hz) - 10_000.0).abs() < EPSILON);
    assert!((hz_to_khz(hz) - 10_000_000.0).abs() < 1.0);
}

// ─── RF band spot-checks ───

#[test]
fn l_band_wavelength() {
    let wl = frequency_to_wavelength(ghz_to_hz(1.575)); // GPS L1
    assert!(wl > 0.19 && wl < 0.20, "GPS L1 λ ≈ 19cm, got {wl}");
}

#[test]
fn s_band_wavelength() {
    let wl = frequency_to_wavelength(ghz_to_hz(2.2)); // deep space downlink
    assert!(wl > 0.13 && wl < 0.14, "S-band λ ≈ 13.6cm, got {wl}");
}

#[test]
fn x_band_wavelength() {
    let wl = frequency_to_wavelength(ghz_to_hz(10.0));
    assert!((wl - 0.0299792458).abs() < 1e-6, "X-band λ ≈ 3cm, got {wl}");
}

#[test]
fn ka_band_wavelength() {
    let wl = frequency_to_wavelength(ghz_to_hz(30.0));
    assert!(wl > 0.009 && wl < 0.011, "Ka-band λ ≈ 1cm, got {wl}");
}

#[test]
fn v_band_wavelength() {
    let wl = frequency_to_wavelength(ghz_to_hz(60.0)); // oxygen absorption
    assert!(wl > 0.004 && wl < 0.006, "V-band λ ≈ 5mm, got {wl}");
}

// ─── Zero and small value behavior ───

#[test]
fn zero_frequency_conversions() {
    assert_eq!(ghz_to_mhz(0.0), 0.0);
    assert_eq!(mhz_to_ghz(0.0), 0.0);
    assert_eq!(hz_to_khz(0.0), 0.0);
    assert_eq!(thz_to_ghz(0.0), 0.0);
}

#[test]
fn very_small_frequency() {
    // 1 Hz = 1e-9 GHz
    let ghz = hz_to_ghz(1.0);
    assert!((ghz - 1e-9).abs() < 1e-15);
}

#[test]
fn very_large_frequency() {
    // 300 GHz = 0.3 THz (edge of microwave/terahertz boundary)
    let thz = ghz_to_thz(300.0);
    assert!((thz - 0.3).abs() < EPSILON);
    let wl = frequency_to_wavelength(ghz_to_hz(300.0));
    assert!(wl > 0.0009 && wl < 0.0011, "300 GHz λ ≈ 1mm, got {wl}");
}
