# touchstone 0.17.0

- Update the RF conversion dependency to `rfconversions` 0.8, including its numerically stable noise conversions and additional scalar helpers.
- Move development to [the four-crate RF workspace](https://github.com/iancleary/rf-crates), preserving this crate's name, MIT license, and independent version.
- Verify the existing parser, network-analysis, CLI, library-only, and package workflows through coordinated workspace CI.

This minor release establishes the monorepo dependency and release line; it does not introduce another Touchstone parser or change S-parameter port numbering.
