# gainlineup 0.24.0

- Combine independent cascade noise powers with `rfconversions::power::add_power_db`, avoiding overflow and underflow from intermediate conversion to watts.
- Preserve finite logarithmic noise and SNR through a noiseless unity-gain stage at stress inputs of ±4000 dBm; retain the public cascade regression.
- Update dependencies to `rfconversions` 0.8 and `touchstone` 0.17. Use Touchstone's public `frequencies()` accessor in the CLI adapter.
- Move development to [the four-crate RF workspace](https://github.com/iancleary/rf-crates), preserving this crate's name, MIT license, and independent version.

The RF block model and output-referred compression conventions are unchanged. Existing minimum-Rust-version metadata is unchanged; release checks use stable Rust.
