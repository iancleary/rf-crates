#![warn(missing_docs)]
//! RF engineering unit conversions for power, frequency, noise, and compression point analysis.
//!
//! Use this crate for scalar RF math and unit normalization: dBm, dBW, watts,
//! dB/linear ratios, frequency/wavelength, noise figure/factor/temperature, kTB
//! noise power, G/T, N0, and P1dB conversions.
//!
//! For ordered RF hardware chains, use `gainlineup`. For `.sNp` S-parameter data,
//! use `touchstone`. For end-to-end radio link budgets, BER, margin, orbit,
//! Doppler, PFD, or modulation workflows, use `linkbudget`.

/// Physical constants used by the conversion routines.
pub mod constants;
/// Frequency and wavelength conversions.
pub mod frequency;
/// Noise figure, noise factor, noise temperature, and thermal noise conversions.
pub mod noise;
/// P1dB compression point conversion helpers.
pub mod p1db;
/// Power conversions including watts, dBm, dBW, and linear ratios.
pub mod power;
