/// Speed of light in vacuum, in meters per second.
///
/// # Examples
///
/// ```
/// use rfconversions::constants::SPEED_OF_LIGHT;
/// assert_eq!(SPEED_OF_LIGHT, 299_792_458.0);
/// ```
pub const SPEED_OF_LIGHT: f64 = 299792458.0;

/// Boltzmann's constant in joules per kelvin (J/K).
///
/// Used in thermal noise calculations: P = kTB.
///
/// # Examples
///
/// ```
/// use rfconversions::constants::BOLTZMANN;
/// assert!((BOLTZMANN - 1.380649e-23).abs() < 1e-29);
/// ```
pub const BOLTZMANN: f64 = 1.380649e-23;

/// Standard reference temperature in kelvin (290 K).
///
/// IEEE standard reference temperature used in noise figure definitions.
///
/// # Examples
///
/// ```
/// use rfconversions::constants::T0;
/// assert_eq!(T0, 290.0);
/// ```
pub const T0: f64 = 290.0;

#[cfg(test)]
mod tests {

    #[test]
    fn speed_of_light() {
        use super::SPEED_OF_LIGHT;
        assert_eq!(299792458.0, SPEED_OF_LIGHT);
    }

    #[test]
    fn boltzmann_constant() {
        use super::BOLTZMANN;
        assert!((BOLTZMANN - 1.380649e-23).abs() < 1e-29);
    }

    #[test]
    fn reference_temperature() {
        use super::T0;
        assert_eq!(290.0, T0);
    }
}
