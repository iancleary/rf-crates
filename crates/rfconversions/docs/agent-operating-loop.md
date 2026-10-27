# Agent operating loop

`rfconversions` is a small public Rust crate for scalar RF engineering
conversions. Its agent ergonomics come from keeping the API explicit, examples
copyable, and tests close to the formulas they protect.

## System shape

The crate is intentionally a leaf dependency. Use it for unit normalization and
single-formula helpers:

- power: watts, milliwatts, dBm, dBW, and dB/linear power ratios
- frequency: Hz/kHz/MHz/GHz/THz scaling and vacuum wavelength
- noise: noise figure, noise factor, equivalent noise temperature, kTB, Friis
  cascade, G/T, and N0
- P1dB: input/output compression-point translation and cascade helpers
- constants: shared physical constants used by the formulas

Do not turn this crate into system simulation. Ordered RF hardware chains belong
in `gainlineup`, S-parameter parsing belongs in `touchstone`, and complete link
budgets belong in `linkbudget`.

## Change loop

1. Identify the owning module before editing.
2. Preserve the public scalar-function style unless a change has a clear
   compatibility reason.
3. Add or update Rustdoc examples for public helpers.
4. Keep README examples and the API summary aligned with public functions.
5. Add focused tests where future agents will look for the behavior:
   - README snippet parity: `tests/readme_examples.rs`
   - RF engineering scenarios: `tests/practical_scenarios.rs`
   - unit grids and round trips: focused integration tests or module tests
6. Run the narrowest useful verification, then `just check` for release-facing
   changes.

## Formula invariants

- dB values are logarithmic. Add and subtract them only when the formula is
  already in the dB domain.
- Convert through `power::db_to_linear`, `power::linear_to_db`, or the noise
  helpers before multiplying ratios, averaging linear values, or applying
  Friis-style formulas.
- Frequency inputs to wavelength helpers are in Hz; wavelengths are in meters.
- Noise temperature conversions use `constants::T0` as the 290 K reference.
- Thermal noise uses `constants::BOLTZMANN`.
- Preserve documented panic behavior for empty cascade inputs unless making an
  explicit API-breaking change.

## Documentation surfaces

- `README.md` is the crates.io-facing guide and public API summary.
- Rustdoc on public functions should carry the shortest correct formula context
  and a runnable example.
- `AGENTS.md` is the primary agent instruction file.
- `CLAUDE.md` mirrors agent-facing guidance for Claude Code users; keep workflow
  and version-independent guidance consistent across both files.
- `docs/release.md` owns the release contract.

## Verification

Use `git diff --check` for every change. For docs-only edits, run `cargo fmt
--all -- --check` when Rust snippets or command guidance changed. For public API
or formula behavior, run:

```bash
just check
```
