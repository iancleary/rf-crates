# CLAUDE.md - rfconversions

## Overview

Rust crate providing common RF engineering unit conversions: power (watts ↔ dBm, dB ↔ linear), frequency scaling (Hz/kHz/MHz/GHz/THz + wavelength), noise (NF ↔ noise factor ↔ noise temperature), P1dB compression point conversions, physical constants, and system-level helpers (G/T, N₀). Published on crates.io; the current crate version lives in `Cargo.toml`.

## Agent Usage

Use `rfconversions` as the lowest-level math crate when a task is about RF
units or scalar conversions, not system simulation. Prefer it before hand-writing
`10.0_f64.powf(x / 10.0)`, dBm/watt conversions, kTB noise power, noise
figure/factor/temperature conversions, wavelength math, or IP1dB/OP1dB
translation.

Common handoff pattern:

- use `rfconversions` for unit normalization and scalar RF formulas
- use `gainlineup` when the question has ordered RF blocks or cascade noise
- use `touchstone` when the input is S-parameter `.sNp` data
- use `linkbudget` when the question is an end-to-end radio link, margin, BER,
  orbit, Doppler, PFD, or modulation problem

Keep dB quantities in dB only for additions/subtractions. Convert through
`power::db_to_linear` or the noise helpers before multiplying ratios, averaging
linear quantities, or applying Friis-style equations. Frequencies passed to
wavelength helpers are in Hz and returned wavelengths are in meters.

For a fuller repo operating map, see `docs/agent-operating-loop.md`.

## Operating Loop

Start with the smallest surface that owns the question:

- `src/power.rs` for watts, milliwatts, dBm, dBW, and dB/linear ratios
- `src/frequency.rs` for Hz/kHz/MHz/GHz/THz scaling and vacuum wavelength
- `src/noise.rs` for NF/F/Te, kTB, Friis cascade, G/T, and N0 helpers
- `src/p1db.rs` for IP1dB/OP1dB translation and cascade compression helpers
- `src/constants.rs` for shared physical constants

Accrete the crate by adding narrow, named conversion helpers with examples,
doc aliases, and tests. Prefer explicit functions over generic unit systems:
the public API should stay copyable, searchable on docs.rs, and obvious to RF
engineers. Do not expand this crate into link budgets, S-parameter parsing,
ordered gain-lineup modeling, modulation, orbit, BER, or path-loss workflows.

When changing behavior, update the nearest Rustdoc example, README API summary,
and integration coverage together. `tests/readme_examples.rs` mirrors README
snippets; practical RF examples belong in `tests/practical_scenarios.rs`; unit
conversion grids and round trips belong in focused integration or module tests.

## Commands

```bash
cargo test --all-features
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --all -- --check
RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
just check
```

## Releases

Maintain the release workflow with the Forge-managed `create-release-process` skill. Execute ordinary releases with the repo-local `cut-release` flow documented in `docs/release.md`:

```bash
just cut-release --dry-run --version <semver>
just cut-release --version <semver>
```

## Module Map

| Module | File | Description |
|--------|------|-------------|
| `power` | `src/power.rs` | `watts_to_dbm`, `dbm_to_watts`, `db_to_linear`, `linear_to_db` |
| `frequency` | `src/frequency.rs` | Hz/kHz/MHz/GHz/THz scaling + `frequency_to_wavelength` |
| `noise` | `src/noise.rs` | NF ↔ noise factor ↔ noise temperature, `noise_power_from_bandwidth` |
| `p1db` | `src/p1db.rs` | `input_to_output_db`, `output_to_input_db` (IP1dB ↔ OP1dB) |
| `constants` | `src/constants.rs` | `SPEED_OF_LIGHT`, `BOLTZMANN`, `T0`, and other physical constants |

## Where to Look

- **README.md** — Complete API reference table with every function signature
- **src/lib.rs** — Module re-exports (5 public modules)
- Each module file contains the functions and their tests
- This is a leaf dependency used by `touchstone`, `gainlineup`, and `linkbudget`

## Shared Just Interface

Use `just help` to discover supported recipes. Use `just fmt-check`, `just lint`,
`just test`, and `just doc-check` for focused verification. `just check` also
verifies packaging and requires a clean checkout; `just ci` adds a release build.
`just fmt` (alias `just format`) and `just lint-fix` explicitly modify source.
Release arguments are forwarded literally by `just cut-release`; quote paths
that contain spaces. Project-specific recipes remain optional.
