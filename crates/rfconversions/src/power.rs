/// Convert watts to dBm.
///
/// # Examples
///
/// ```
/// use rfconversions::power::watts_to_dbm;
/// assert_eq!(watts_to_dbm(1.0), 30.0);
/// ```
#[doc(alias = "dBm")]
#[must_use]
pub fn watts_to_dbm(watts: f64) -> f64 {
    10.0 * (watts.log10() + 3.0)
}

/// Convert dBm to watts.
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbm_to_watts;
/// assert_eq!(dbm_to_watts(30.0), 1.0);
/// ```
#[doc(alias = "dBm")]
#[must_use]
pub fn dbm_to_watts(dbm: f64) -> f64 {
    10.0_f64.powf((dbm - 30.0) / 10.0)
}

/// Convert a dB value to its linear (power) ratio.
///
/// # Examples
///
/// ```
/// use rfconversions::power::db_to_linear;
/// assert_eq!(db_to_linear(30.0), 1000.0);
/// ```
#[doc(alias = "dB")]
#[doc(alias = "decibel")]
#[must_use]
pub fn db_to_linear(value: f64) -> f64 {
    10.0_f64.powf(value / 10.0)
}

/// Convert a linear power ratio to dB.
///
/// # Examples
///
/// ```
/// use rfconversions::power::linear_to_db;
/// assert_eq!(linear_to_db(1000.0), 30.0);
/// ```
#[doc(alias = "dB")]
#[doc(alias = "decibel")]
#[must_use]
pub fn linear_to_db(value: f64) -> f64 {
    10.0 * f64::log10(value)
}

/// Convert dBm to milliwatts.
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbm_to_milliwatts;
/// assert_eq!(dbm_to_milliwatts(0.0), 1.0);
/// ```
#[doc(alias = "dBm")]
#[doc(alias = "mW")]
#[must_use]
pub fn dbm_to_milliwatts(dbm: f64) -> f64 {
    10.0_f64.powf(dbm / 10.0)
}

/// Convert milliwatts to dBm.
///
/// # Examples
///
/// ```
/// use rfconversions::power::milliwatts_to_dbm;
/// assert_eq!(milliwatts_to_dbm(1.0), 0.0);
/// ```
#[doc(alias = "dBm")]
#[doc(alias = "mW")]
#[must_use]
pub fn milliwatts_to_dbm(mw: f64) -> f64 {
    10.0 * mw.log10()
}

/// Convert watts to dBW.
///
/// # Examples
///
/// ```
/// use rfconversions::power::watts_to_dbw;
/// assert_eq!(watts_to_dbw(1.0), 0.0);
/// assert_eq!(watts_to_dbw(100.0), 20.0);
/// ```
#[doc(alias = "dBW")]
#[must_use]
pub fn watts_to_dbw(watts: f64) -> f64 {
    10.0 * watts.log10()
}

/// Convert dBW to watts.
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbw_to_watts;
/// assert_eq!(dbw_to_watts(0.0), 1.0);
/// ```
#[doc(alias = "dBW")]
#[must_use]
pub fn dbw_to_watts(dbw: f64) -> f64 {
    10.0_f64.powf(dbw / 10.0)
}

/// Convert dBm to dBW (subtract 30).
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbm_to_dbw;
/// assert_eq!(dbm_to_dbw(30.0), 0.0);
/// ```
#[doc(alias = "dBm")]
#[doc(alias = "dBW")]
#[must_use]
pub fn dbm_to_dbw(dbm: f64) -> f64 {
    dbm - 30.0
}

/// Convert dBW to dBm (add 30).
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbw_to_dbm;
/// assert_eq!(dbw_to_dbm(0.0), 30.0);
/// ```
#[doc(alias = "dBm")]
#[doc(alias = "dBW")]
#[must_use]
pub fn dbw_to_dbm(dbw: f64) -> f64 {
    dbw + 30.0
}

/// Convert milliwatts to dBW.
///
/// # Examples
///
/// ```
/// use rfconversions::power::milliwatts_to_dbw;
/// assert_eq!(milliwatts_to_dbw(1000.0), 0.0);
/// ```
#[doc(alias = "mW")]
#[doc(alias = "dBW")]
#[must_use]
pub fn milliwatts_to_dbw(mw: f64) -> f64 {
    10.0 * (mw / 1000.0).log10()
}

/// Convert dBW to milliwatts.
///
/// # Examples
///
/// ```
/// use rfconversions::power::dbw_to_milliwatts;
/// assert_eq!(dbw_to_milliwatts(0.0), 1000.0);
/// ```
#[doc(alias = "mW")]
#[doc(alias = "dBW")]
#[must_use]
pub fn dbw_to_milliwatts(dbw: f64) -> f64 {
    10.0_f64.powf(dbw / 10.0) * 1000.0
}

#[cfg(test)]
mod tests {

    #[test]
    fn watts_to_dbm() {
        let watts: f64 = 1.0;

        let dbm: f64 = super::watts_to_dbm(watts);

        assert_eq!(30.0, dbm);
    }

    #[test]
    fn another_watts_to_dbm() {
        let watts: f64 = 20.0;

        let dbm: f64 = super::watts_to_dbm(watts);

        // not worrying about floating point precision here
        assert_eq!(43.01029995663981, dbm);
    }

    #[test]
    fn dbm_to_watts() {
        // not worrying about floating point precision here
        let dbm: f64 = 43.010_299_956_639_805;

        let watts: f64 = super::dbm_to_watts(dbm);

        // not worrying about floating point precision here
        assert_eq!(19.99999999999997, watts);
    }

    #[test]
    fn another_dbm_to_watts() {
        let dbm: f64 = 30.0;

        let watts: f64 = super::dbm_to_watts(dbm);

        assert_eq!(1.0, watts);
    }

    #[test]
    fn db_to_linear() {
        let db: f64 = 30.0;

        let linear: f64 = super::db_to_linear(db);

        assert_eq!(1000.0, linear);
    }

    #[test]
    fn another_db_to_linear() {
        let db: f64 = -10.0;

        let linear: f64 = super::db_to_linear(db);

        assert_eq!(0.1, linear);
    }
    #[test]
    fn another_db_to_linear_2() {
        let db: f64 = -13.0;

        let linear: f64 = super::db_to_linear(db);

        // -3.0 dB isn't exactly half
        // therefore -13 dB isn't exactly 1/20
        assert_eq!(0.05011872336272722, linear);
    }

    #[test]
    fn linear_to_db() {
        let linear: f64 = 1000.0;

        let db: f64 = super::linear_to_db(linear);

        assert_eq!(30.0, db);
    }

    #[test]
    fn another_linear_to_db() {
        let linear: f64 = 0.1;

        let db: f64 = super::linear_to_db(linear);

        assert_eq!(-10.0, db);
    }
    #[test]
    fn another_linear_to_db_2() {
        let linear: f64 = 0.05011872336272722;

        let db: f64 = super::linear_to_db(linear);

        // -3.0 dB isn't exactly half
        // therefore -13 dB isn't exactly 1/20
        assert_eq!(-13.0, db);
    }

    #[test]
    fn dbm_to_milliwatts_zero_dbm() {
        let mw: f64 = super::dbm_to_milliwatts(0.0);
        assert_eq!(1.0, mw);
    }

    #[test]
    fn milliwatts_to_dbm_one_mw() {
        let dbm: f64 = super::milliwatts_to_dbm(1.0);
        assert_eq!(0.0, dbm);
    }

    #[test]
    fn watts_to_dbm_zero_watts() {
        let dbm: f64 = super::watts_to_dbm(0.0);
        assert!(dbm.is_infinite() && dbm < 0.0);
    }

    #[test]
    fn roundtrip_dbm_watts_dbm() {
        let original_dbm: f64 = 23.0;
        let watts: f64 = super::dbm_to_watts(original_dbm);
        let result_dbm: f64 = super::watts_to_dbm(watts);
        assert!((original_dbm - result_dbm).abs() < 1e-10);
    }

    #[test]
    fn roundtrip_linear_db_linear() {
        let original_linear: f64 = 50.0;
        let db: f64 = super::linear_to_db(original_linear);
        let result_linear: f64 = super::db_to_linear(db);
        assert!((original_linear - result_linear).abs() < 1e-10);
    }

    #[test]
    fn negative_db_values() {
        let db: f64 = -3.010299956639812;
        let linear: f64 = super::db_to_linear(db);
        assert!((0.5 - linear).abs() < 1e-10);
    }

    #[test]
    fn watts_to_dbw_one_watt() {
        assert_eq!(0.0, super::watts_to_dbw(1.0));
    }

    #[test]
    fn watts_to_dbw_hundred_watts() {
        assert_eq!(20.0, super::watts_to_dbw(100.0));
    }

    #[test]
    fn dbw_to_watts_zero_dbw() {
        assert_eq!(1.0, super::dbw_to_watts(0.0));
    }

    #[test]
    fn dbw_to_watts_ten_dbw() {
        assert_eq!(10.0, super::dbw_to_watts(10.0));
    }

    #[test]
    fn dbm_to_dbw_30dbm() {
        assert_eq!(0.0, super::dbm_to_dbw(30.0));
    }

    #[test]
    fn dbw_to_dbm_zero_dbw() {
        assert_eq!(30.0, super::dbw_to_dbm(0.0));
    }

    #[test]
    fn roundtrip_dbw_watts_dbw() {
        let original: f64 = 13.0;
        let watts: f64 = super::dbw_to_watts(original);
        let result: f64 = super::watts_to_dbw(watts);
        assert!((original - result).abs() < 1e-10);
    }

    #[test]
    fn milliwatts_to_dbw_1000mw() {
        assert_eq!(0.0, super::milliwatts_to_dbw(1000.0));
    }

    #[test]
    fn milliwatts_to_dbw_1mw() {
        let result = (super::milliwatts_to_dbw(1.0) * 1e5).round() / 1e5;
        assert_eq!(-30.0, result);
    }

    #[test]
    fn dbw_to_milliwatts_zero_dbw() {
        assert_eq!(1000.0, super::dbw_to_milliwatts(0.0));
    }

    #[test]
    fn dbw_to_milliwatts_neg30_dbw() {
        let result = (super::dbw_to_milliwatts(-30.0) * 1e5).round() / 1e5;
        assert_eq!(1.0, result);
    }

    #[test]
    fn roundtrip_mw_dbw_mw() {
        let original: f64 = 500.0;
        let dbw: f64 = super::milliwatts_to_dbw(original);
        let result: f64 = super::dbw_to_milliwatts(dbw);
        assert!((original - result).abs() < 1e-10);
    }

    #[test]
    fn roundtrip_dbm_dbw_dbm() {
        let original: f64 = 23.0;
        let dbw: f64 = super::dbm_to_dbw(original);
        let result: f64 = super::dbw_to_dbm(dbw);
        assert_eq!(original, result);
    }
}
