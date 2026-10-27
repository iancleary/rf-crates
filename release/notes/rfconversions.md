# rfconversions 0.8.0

- Add `power::add_power_db` for stable addition of independent logarithmic powers, including finite inputs outside the range representable in watts.
- Improve noise-figure/equivalent-noise-temperature conversions near zero with direct `expm1` and `ln_1p` formulations.
- Add `noise::noise_temperature_from_y_factor` for hot/cold calibration with explicit invalid-input behavior.
- Retain independent Decimal and exact-rational numerical comparisons alongside public integration coverage.
- Move development to [the four-crate RF workspace](https://github.com/iancleary/rf-crates), preserving this crate's name, MIT license, and independent version.

The minor version establishes the monorepo release line. Existing minimum-Rust-version metadata is unchanged; the release checks run stable Rust, not a full historical compiler matrix.
