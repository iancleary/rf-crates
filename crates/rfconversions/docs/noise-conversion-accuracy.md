# Direct noise figure ↔ equivalent temperature

## Mental model

The public helpers in [`src/noise.rs`](../src/noise.rs) keep their `f64 → f64`
signatures and use [`constants::T0`](../src/constants.rs) = 290 K:

```text
Te = T0 × (10^(NF/10) − 1)
NF = 10 × log10(1 + Te/T0)
```

Going through a noise factor near one discards the difference the caller wants.
The direct helpers instead use `exp_m1` and `ln_1p`. The factor APIs still return
rounded factors, and the Friis cascade APIs are unchanged. This is scalar RF
math, not a new temperature convention or system model.

The RF definitions and 290 K reference are in Condon and Ransom,
[*Essential Radio Astronomy*, §3.6.6, equations 3.169–3.171](https://www.cv.nrao.edu/~sransom/web/Ch3.html#S6.SS6).
The numerical motivation is cancellation, discussed in Goldberg's
[authorized reprint, Theorem 4](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html#719).
These substitutions are our application of those sources, not copied numerical
code. Rust documents [`exp_m1`](https://doc.rust-lang.org/std/primitive.f64.html#method.exp_m1)
and [`ln_1p`](https://doc.rust-lang.org/std/primitive.f64.html#method.ln_1p),
including their platform-dependent precision.

## Why range-aware direct formulas

A simple substitution still rounds intermediate scalings unnecessarily:

- For `|NF| < EPSILON`, use `NF × (T0 × ln(10)/10)`. The relative quadratic
  correction to the exponential's linear term is below approximately
  `EPSILON × ln(10)/20`. Combining constants prevents the exponent from
  underflowing before multiplication by T0.
- For `|Te| < T0 × EPSILON`, use `Te × ((10/ln(10))/T0)`.
  The omitted logarithmic correction is below approximately `EPSILON/2`
  relative. Dividing Te first could underflow while the final NF is representable.
  These are rounding-scale approximations, not correctly-rounded guarantees.
- For `Te < −T0/2`, use `ln((Te + T0)/T0)` instead of `ln_1p(Te/T0)`.
  Near the singularity, Sterbenz's lemma makes `Te + T0` exact for finite
  `−2T0 ≤ Te ≤ −T0/2`; division of Te first loses the tiny distance from −T0.
  Below the domain, the negative logarithm argument naturally yields NaN.
- Otherwise use the direct `exp_m1`/`ln_1p` forms. No large-input branch was
  justified by the sampled evidence.

### Observed candidate comparison

The retained probe compares the old composition, simple direct formulas, the
range-aware candidate, and the actual public APIs. References use **exact binary
f64 inputs**, not the decimal spelling alone, with Python stdlib Decimal at
500 and 800 digits. Both precisions must round to the same f64 reference.

| Input → output | Baseline | Simple direct | Chosen range-aware |
| --- | --- | --- | --- |
| NF=1e−16 → K | 0 | 6.677496769682733e−15 | 6.677496769682733e−15 |
| NF=minimum subnormal → K | 0 | 0 | 67 minimum-subnormal units (reference) |
| NF=1e−320 → K, relative error | 1 | about 9.62e−5 | 0 in this sample |
| Te=2e−322 → dB | 0 | 0 | minimum subnormal (reference) |
| Te=next f64 above −290 → dB | −156.53559774527022 | −156.53559774527020 | −157.07717807114128 (reference) |
| Ordinary NF=0.5/3/6, Te=35/290 | small rounding differences | small rounding differences | relative error below 3e−16 in these samples |
| NF=3057.9231756201775 → K | finite | finite | finite; about 8.26e−14 relative error |
| Next f64 NF above that → K | finite (wrong classification) | +∞ | +∞ (reference) |

This is sampled numerical evidence, **not** a full-domain error bound, universal
libm accuracy claim, or speed benchmark. Large exponent argument rounding can
amplify error. Tests use 2e−15 relative tolerance for ordinary normal outputs and
3e−13 near overflow; exact subnormal checks protect cases where even a one-unit
absolute tolerance would incorrectly accept zero. Existing exact transcendental
assertions were replaced by tolerances against the independent references, not
by copying production outputs.

## Domain and compatibility

Physical equivalent receiver temperatures are normally nonnegative. The existing
algebraic extension remains: negative NF gives negative Te; NF=−∞ gives −290 K;
Te=−290 K gives −∞ dB; Te<−290 K (including −∞) gives NaN. Positive infinity
remains positive infinity, and NaN propagates. Both signed-zero **inputs** return
+0 for compatibility. A tiny negative nonzero Te can now produce −0 when the
true negative result underflows; this preserves the result's direction rather
than clamping all underflows. Factor and cascade behavior is untouched.

## Rerun offline

Prerequisites: the repository's Rust toolchain, `uv`, an already installed Python
with stdlib Decimal, and cached Cargo dependencies. Do not install anything for
this procedure. Run from the checkout root; helper resources stay together in
[the skill-owned scripts folder](../.agents/skills/rf-literature-review/scripts/).
The explicit `--extern` selects this checkout, not a separately installed crate.

```bash
CARGO_NET_OFFLINE=true cargo build --all-features
rustfmt --edition 2021 --check .agents/skills/rf-literature-review/scripts/compare_noise.rs
rustc --edition 2021 .agents/skills/rf-literature-review/scripts/compare_noise.rs \
  --extern rfconversions=target/debug/librfconversions.rlib \
  -L dependency=target/debug/deps -o /tmp/compare-noise
uv run --offline --no-project \
  .agents/skills/rf-literature-review/scripts/noise_references.py > /tmp/noise-references.csv
/tmp/compare-noise < /tmp/noise-references.csv
```

The CSV reports values and relative errors; zero and nonfinite references use
classification comparisons. Signed-zero and NaN/infinity contracts are exercised
by [`tests/noise_conversion_accuracy.rs`](../tests/noise_conversion_accuracy.rs),
not inferred from relative error. The probe is diagnostic, not an automatic
accuracy certification. If Decimal precision agreement fails, increase both
precisions before trusting new fixtures. If the crate cannot be found, rebuild
and check whether `CARGO_TARGET_DIR` relocates the paths in the compile command.
Do not replace missing cached dependencies with unapproved downloads.

Verification gates:

```bash
CARGO_NET_OFFLINE=true cargo fmt --all -- --check
CARGO_NET_OFFLINE=true cargo clippy --all-targets --all-features -- -D warnings
CARGO_NET_OFFLINE=true cargo test --all-features
CARGO_NET_OFFLINE=true RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps
# After committing, to include clean-checkout packaging:
CARGO_NET_OFFLINE=true just check
```

To explore an alternative, change only the diagnostic candidate first, regenerate
references, and inspect the demonstrated gap before changing production. Keep
oracle calculations independent from Rust's elementary functions. No production
writes, hardware calibration, dependency installation, or release is part of this
workflow. Reverting the focused conversion change restores baseline rounding;
there is no stored data or migration to roll back.
