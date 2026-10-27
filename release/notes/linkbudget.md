# linkbudget 0.8.0

- Update the RF conversion dependency to `rfconversions` 0.8, including its numerically stable noise conversions and additional scalar helpers.
- Make HTML-output verification independent of an assumed member-local `target/` directory.
- Move development to [the four-crate RF workspace](https://github.com/iancleary/rf-crates), preserving this crate's name, MIT license, and independent version.
- Verify the existing link-budget, CLI, library-only, and package workflows through coordinated workspace CI.

This minor release establishes the monorepo dependency and release line. Existing minimum-Rust-version metadata is unchanged; release checks use stable Rust rather than a full historical compiler matrix.
