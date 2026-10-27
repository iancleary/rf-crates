/// Convert frequency (Hz) to vacuum wavelength (meters).
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::frequency_to_wavelength;
/// let wavelength = frequency_to_wavelength(1.0e9);
/// assert_eq!(wavelength, 0.299792458);
/// ```
#[doc(alias = "lambda")]
#[must_use]
pub fn frequency_to_wavelength(frequency: f64) -> f64 {
    crate::constants::SPEED_OF_LIGHT / frequency
}

/// Convert vacuum wavelength (meters) to frequency (Hz).
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::wavelength_to_frequency;
/// let freq = wavelength_to_frequency(0.299792458);
/// assert_eq!(freq, 1.0e9);
/// ```
#[doc(alias = "lambda")]
#[must_use]
pub fn wavelength_to_frequency(wavelength: f64) -> f64 {
    crate::constants::SPEED_OF_LIGHT / wavelength
}

/// Convert THz to GHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::thz_to_ghz;
/// assert_eq!(thz_to_ghz(1.0), 1000.0);
/// ```
#[doc(alias = "THz")]
#[doc(alias = "GHz")]
#[must_use]
pub fn thz_to_ghz(thz: f64) -> f64 {
    thz * 1e3
}

/// Convert THz to MHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::thz_to_mhz;
/// assert_eq!(thz_to_mhz(1.0), 1_000_000.0);
/// ```
#[doc(alias = "THz")]
#[doc(alias = "MHz")]
#[must_use]
pub fn thz_to_mhz(thz: f64) -> f64 {
    thz * 1e6
}

/// Convert THz to kHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::thz_to_khz;
/// assert_eq!(thz_to_khz(1.0), 1_000_000_000.0);
/// ```
#[doc(alias = "THz")]
#[doc(alias = "kHz")]
#[must_use]
pub fn thz_to_khz(thz: f64) -> f64 {
    thz * 1e9
}

/// Convert THz to Hz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::thz_to_hz;
/// assert_eq!(thz_to_hz(1.0), 1_000_000_000_000.0);
/// ```
#[doc(alias = "THz")]
#[doc(alias = "Hz")]
#[must_use]
pub fn thz_to_hz(thz: f64) -> f64 {
    thz * 1e12
}

/// Convert GHz to THz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::ghz_to_thz;
/// assert_eq!(ghz_to_thz(1000.0), 1.0);
/// ```
#[doc(alias = "GHz")]
#[doc(alias = "THz")]
#[must_use]
pub fn ghz_to_thz(ghz: f64) -> f64 {
    ghz / 1e3
}

/// Convert GHz to MHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::ghz_to_mhz;
/// assert_eq!(ghz_to_mhz(1.0), 1000.0);
/// ```
#[doc(alias = "GHz")]
#[doc(alias = "MHz")]
#[must_use]
pub fn ghz_to_mhz(ghz: f64) -> f64 {
    ghz * 1e3
}

/// Convert GHz to kHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::ghz_to_khz;
/// assert_eq!(ghz_to_khz(1.0), 1_000_000.0);
/// ```
#[doc(alias = "GHz")]
#[doc(alias = "kHz")]
#[must_use]
pub fn ghz_to_khz(ghz: f64) -> f64 {
    ghz * 1e6
}

/// Convert GHz to Hz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::ghz_to_hz;
/// assert_eq!(ghz_to_hz(1.0), 1_000_000_000.0);
/// ```
#[doc(alias = "GHz")]
#[doc(alias = "Hz")]
#[must_use]
pub fn ghz_to_hz(ghz: f64) -> f64 {
    ghz * 1e9
}

/// Convert MHz to THz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::mhz_to_thz;
/// assert_eq!(mhz_to_thz(1_000_000.0), 1.0);
/// ```
#[doc(alias = "MHz")]
#[doc(alias = "THz")]
#[must_use]
pub fn mhz_to_thz(mhz: f64) -> f64 {
    mhz / 1e6
}

/// Convert MHz to GHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::mhz_to_ghz;
/// assert_eq!(mhz_to_ghz(1000.0), 1.0);
/// ```
#[doc(alias = "MHz")]
#[doc(alias = "GHz")]
#[must_use]
pub fn mhz_to_ghz(mhz: f64) -> f64 {
    mhz / 1e3
}

/// Convert MHz to kHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::mhz_to_khz;
/// assert_eq!(mhz_to_khz(1.0), 1000.0);
/// ```
#[doc(alias = "MHz")]
#[doc(alias = "kHz")]
#[must_use]
pub fn mhz_to_khz(mhz: f64) -> f64 {
    mhz * 1e3
}

/// Convert MHz to Hz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::mhz_to_hz;
/// assert_eq!(mhz_to_hz(1.0), 1_000_000.0);
/// ```
#[doc(alias = "MHz")]
#[doc(alias = "Hz")]
#[must_use]
pub fn mhz_to_hz(mhz: f64) -> f64 {
    mhz * 1e6
}

/// Convert kHz to THz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::khz_to_thz;
/// assert_eq!(khz_to_thz(1_000_000_000.0), 1.0);
/// ```
#[doc(alias = "kHz")]
#[doc(alias = "THz")]
#[must_use]
pub fn khz_to_thz(khz: f64) -> f64 {
    khz / 1e9
}

/// Convert kHz to GHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::khz_to_ghz;
/// assert_eq!(khz_to_ghz(1_000_000.0), 1.0);
/// ```
#[doc(alias = "kHz")]
#[doc(alias = "GHz")]
#[must_use]
pub fn khz_to_ghz(khz: f64) -> f64 {
    khz / 1e6
}

/// Convert kHz to MHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::khz_to_mhz;
/// assert_eq!(khz_to_mhz(1000.0), 1.0);
/// ```
#[doc(alias = "kHz")]
#[doc(alias = "MHz")]
#[must_use]
pub fn khz_to_mhz(khz: f64) -> f64 {
    khz / 1e3
}

/// Convert kHz to Hz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::khz_to_hz;
/// assert_eq!(khz_to_hz(1.0), 1000.0);
/// ```
#[doc(alias = "kHz")]
#[doc(alias = "Hz")]
#[must_use]
pub fn khz_to_hz(khz: f64) -> f64 {
    khz * 1e3
}

/// Convert Hz to THz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::hz_to_thz;
/// assert_eq!(hz_to_thz(1_000_000_000_000.0), 1.0);
/// ```
#[doc(alias = "Hz")]
#[doc(alias = "THz")]
#[must_use]
pub fn hz_to_thz(hz: f64) -> f64 {
    hz / 1e12
}

/// Convert Hz to GHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::hz_to_ghz;
/// assert_eq!(hz_to_ghz(1_000_000_000.0), 1.0);
/// ```
#[doc(alias = "Hz")]
#[doc(alias = "GHz")]
#[must_use]
pub fn hz_to_ghz(hz: f64) -> f64 {
    hz / 1e9
}

/// Convert Hz to MHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::hz_to_mhz;
/// assert_eq!(hz_to_mhz(1_000_000.0), 1.0);
/// ```
#[doc(alias = "Hz")]
#[doc(alias = "MHz")]
#[must_use]
pub fn hz_to_mhz(hz: f64) -> f64 {
    hz / 1e6
}

/// Convert Hz to kHz.
///
/// # Examples
///
/// ```
/// use rfconversions::frequency::hz_to_khz;
/// assert_eq!(hz_to_khz(1000.0), 1.0);
/// ```
#[doc(alias = "Hz")]
#[doc(alias = "kHz")]
#[must_use]
pub fn hz_to_khz(hz: f64) -> f64 {
    hz / 1e3
}

#[cfg(test)]
mod tests {
    #[test]
    fn frequency_to_wavelength_one_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 1.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.299792458, wavelength);
    }

    #[test]
    fn frequency_to_wavelength_twenty_seven_point_five_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 27.5 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.010901543927272727, wavelength);
    }

    #[test]
    fn frequency_to_wavelength_thirty_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 30.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);

        assert_eq!(0.009993081933333333, wavelength);
    }

    #[test]
    fn test_frequency_conversions_hz_khz_mhz_ghz_thz() {
        // Values chosen such that they all convert to 1 THz or 1,000,000,000,000 Hz
        let thz = 1.0;
        let ghz = 1000.0;
        let mhz = 1_000_000.0;
        let khz = 1_000_000_000.0;
        let hz = 1_000_000_000_000.0;
        assert_eq!(super::thz_to_ghz(thz), ghz);
        assert_eq!(super::thz_to_mhz(thz), mhz);
        assert_eq!(super::thz_to_khz(thz), khz);
        assert_eq!(super::thz_to_hz(thz), hz);

        assert_eq!(super::ghz_to_thz(ghz), thz);
        assert_eq!(super::ghz_to_mhz(ghz), mhz);
        assert_eq!(super::ghz_to_khz(ghz), khz);
        assert_eq!(super::ghz_to_hz(ghz), hz);

        assert_eq!(super::mhz_to_thz(mhz), thz);
        assert_eq!(super::mhz_to_ghz(mhz), ghz);
        assert_eq!(super::mhz_to_khz(mhz), khz);
        assert_eq!(super::mhz_to_hz(mhz), hz);

        assert_eq!(super::khz_to_thz(khz), thz);
        assert_eq!(super::khz_to_ghz(khz), ghz);
        assert_eq!(super::khz_to_mhz(khz), mhz);
        assert_eq!(super::khz_to_hz(khz), hz);

        assert_eq!(super::hz_to_thz(hz), thz);
        assert_eq!(super::hz_to_ghz(hz), ghz);
        assert_eq!(super::hz_to_mhz(hz), mhz);
        assert_eq!(super::hz_to_khz(hz), khz);
    }

    #[test]
    fn frequency_conversions_non_unity_thz() {
        // Use 2.5 THz to catch mutants that return 1.0
        let thz = 2.5;
        let ghz = 2_500.0;
        let mhz = 2_500_000.0;
        let khz = 2_500_000_000.0;
        let hz = 2_500_000_000_000.0;

        assert_eq!(super::mhz_to_thz(mhz), thz);
        assert_eq!(super::khz_to_thz(khz), thz);
        assert_eq!(super::hz_to_thz(hz), thz);

        assert_eq!(super::thz_to_ghz(thz), ghz);
        assert_eq!(super::thz_to_mhz(thz), mhz);
        assert_eq!(super::thz_to_khz(thz), khz);
        assert_eq!(super::thz_to_hz(thz), hz);
    }

    #[test]
    fn wavelength_to_frequency_roundtrip_one_gigahertz() {
        let base: f64 = 10.0;
        let frequency: f64 = 1.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);
        let result: f64 = super::wavelength_to_frequency(wavelength);

        assert_eq!(frequency, result);
    }

    #[test]
    fn wavelength_to_frequency_ka_band_30_ghz() {
        let base: f64 = 10.0;
        let frequency: f64 = 30.0 * base.powf(9.0);

        let wavelength: f64 = super::frequency_to_wavelength(frequency);
        assert_eq!(0.009993081933333333, wavelength);

        let result: f64 = super::wavelength_to_frequency(wavelength);
        assert_eq!(frequency, result);
    }
}
