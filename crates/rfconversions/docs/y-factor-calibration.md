# Hot/cold Y-factor calibration

## Mental model and caller workflow

A linear receiver with unchanged gain and bandwidth produces detected noise
powers proportional to `Th + Te` and `Tc + Te`. The effective hot and cold load
temperatures must refer to the receiver input reference plane, in kelvin. Their
ratio eliminates the common gain and bandwidth:

```text
Y = Phot / Pcold = (Th + Te) / (Tc + Te)
Te = (Th - Y*Tc)/(Y - 1) = (Th - Tc)/(Y - 1) - Tc
```

The first relation is from Condon & Ransom, *Essential Radio Astronomy* (2016),
[NRAO online §3.6.6, equations (3.165)–(3.168)](https://www.cv.nrao.edu/~sransom/web/Ch3.html#S6.SS6),
[DOI 10.1515/9781400881161](https://doi.org/10.1515/9781400881161).
Equations (3.169)–(3.171) describe F, Te, NF and the 290 K reference. The solved
forms above and the numerical algorithm below are our derivation, not a source
implementation. Equation numbers refer to the online edition.

The public helper in [`src/noise.rs`](../src/noise.rs) is
`noise::noise_temperature_from_y_factor(y, hot_kelvin, cold_kelvin) -> f64`.
For measured dBm readings, first call
`power::db_to_linear(hot_dbm - cold_dbm)`; this uses **10**, not 20, in the dB
conversion. Y is a detected **power** ratio, not an RF voltage-amplitude ratio.
The source's detector voltages are proportional to power. Feed Te into
`noise::noise_figure_from_noise_temperature` for NF at the existing 290 K
reference. The [README example](../README.md#hotcold-receiver-calibration) and
[`tests/readme_examples.rs`](../tests/readme_examples.rs) exercise this complete
scalar handoff with the unchanged existing conversion APIs: approximately
3.0103 dB hot/cold difference gives Y = 2, Te = 136 K for 290/77 K loads,
and NF = 1.6701160120376283 dB.

## Domain and operational limits

All arguments must be finite, with `Th > Tc >= 0` and `Y > 1`. Otherwise the
helper returns NaN, including Y = 1 and any infinite input. Negative zero is a
valid cold temperature. Negative inferred Te is deliberately not clamped:
300/100 K loads at Y = 5 give -50 K, a diagnostic inconsistent with a
nonnegative receiver temperature under this model. At Y = 3 they give zero.
Output infinity is permitted for mathematical overflow.

This is a classical thermal-noise scalar calibration, not a driver, device
state model, uncertainty framework, or correction pipeline. It assumes equal
linear gain/bandwidth and input-referred effective load temperatures. Mismatch,
source loss, gain drift, detector nonlinearity, and quantum-regime noise require
separate treatment. No hardware calibration has been verified here.

Near Y = 1 the inverse problem is ill-conditioned:

```text
dTe/dY = -(Th - Tc)/(Y - 1)^2
```

At Th = 290 K, Tc = 77 K, Y = 1.01, an error of 0.0001 in Y corresponds to
about 213 K first-order error. Numerical rearrangement cannot repair that
measurement sensitivity or recover precision already lost in the readings.
If Te is negative, check the power-ratio convention, effective load temperatures,
reference plane, and gain stability; do not silently clip the diagnostic.

## Numerical alternatives and choice

The ordinary analytic points alone do not discriminate between algorithms.
The retained probe compares six variants against exact rational arithmetic on
**the exact input f64 values**, not idealized decimal readings:

| Variant | Observed limitation |
| --- | --- |
| Textbook `(Th-Y*Tc)/(Y-1)` | Product overflow; cancellation near zero |
| Rearranged `(Th-Tc)/(Y-1)-Tc` | Quotient can overflow before a finite subtraction; cancellation near zero |
| Fused `fma(-Y,Tc,Th)/(Y-1)` | Negative numerator can overflow although the quotient is finite; subnormal numerator rounding |
| Fused for Y <= 2, rearranged otherwise | Avoids the range failures sampled, but loses near-zero accuracy above 2 |
| Fused, falling back to rearranged on a nonfinite numerator | Avoids range failures, but subnormal numerator rounding remains |
| **Scaled fused with nonfinite-numerator fallback (selected)** | Worst observed error one ULP; not a proof of correct rounding everywhere |

Concrete failures:

- Y = 1.5, Th = 1.5e308, Tc = 5e307: Te rounds to 1.5e308, but the
  rearranged quotient overflows before subtracting Tc.
- Y = 1e308, Th = 300, Tc = 100: Te rounds to -100, but the fused numerator
  alone overflows negatively.
- The f64 immediately above Y = 3, with 300/100 K loads, has exact-input Te
  rounding to -2.2204460492503124e-14. The piecewise/rearranged version gives
  -2.842170943040401e-14. FMA preserves the near-zero difference.
- Y = 1.5, Th = twice the minimum positive subnormal, Tc = the minimum
  subnormal: Te is one minimum subnormal. Unscaled FMA rounds the numerator
  to zero before division.

The selected algorithm scales both temperatures by **2^512 when Th < 1 K**,
computes a fused numerator, divides, then undoes the scale. This is exact
power-of-two input scaling, not a measurement correction. Even the minimum
subnormal is brought into the normal range. The scaled hot temperature is
less than 2^512; dividing by the smallest representable positive Y-1 (2^-52)
cannot overflow. With unscaled Th >= 1, near-cancellation numerator differences
are not at the subnormal temperature scale. The final downscaling can still
round a genuinely tiny answer to zero.

If the fused numerator is nonfinite, use the rearranged form instead. This
fallback has a range justification, not an empirical arbitrary cutoff: hot is
positive and greater than cold, so the numerator cannot overflow positively.
Negative overflow requires Y > 2; then Y-1 >= 1 and `(hot-cold)/(Y-1)` is
bounded by hot. The final result is bounded below by -cold. Thus the fallback
cannot introduce avoidable overflow. The fused finite-numerator path also
avoids the rearranged form's overflowing quotient example.

The 4,011-case run includes analytic points, neighbors of Y = 1/2/3, subnormal
loads, nearby load temperatures, extreme finite Y and temperatures, 3,000
seeded log-range samples (one rounded degenerate pair skipped), and 1,000
rounded-ratio near-zero cases. Results, in the table's order:

```text
not correctly rounded:       2181, 1435, 626, 1183, 352, 313
nonfinite classification mismatches: 274, 1, 274, 0, 0, 0
```

Within these 4,011 samples only, the selected version had no nonfinite
classification mismatches and its worst finite distance was one ULP. This is
sampled evidence, not exhaustive f64 verification, an all-domain ULP guarantee,
or a performance claim. The focused integration tests pin independent analytic and
exact-rational expectations rather than duplicating the implementation.

An actual-API independent review also identified an overflow-boundary limit
outside that sample: `noise_temperature_from_y_factor(1.1,
1.7976931348623175e307, 1e291)` returns positive infinity, while the correctly
rounded exact-input `Fraction` oracle returns `f64::MAX`. The exact mathematical
result is above `f64::MAX` by about 0.448854 ULP, but below the nearest-rounding
overflow threshold (half an ULP above `f64::MAX`). Rounding the finite FMA
numerator can shift the output classification there. This is allowed by the
existing mathematical-overflow contract, but there is no universal guarantee of
a correctly rounded overflow boundary. This limitation is not an additional
case in the sampled classification gate.

## Rerun the offline evidence

Requires cached Cargo dependencies, an available Rust toolchain and Python via
uv; no package installation or Python dependencies are needed. From the repository
root (temporary outputs are outside the repository):

```bash
CARGO_NET_OFFLINE=true cargo build
rustc --edition 2021 .agents/skills/rf-literature-review/scripts/compare_y_factor.rs \
  --extern rfconversions=target/debug/librfconversions.rlib \
  -L dependency=target/debug/deps -o /tmp/rf-y-factor-compare
uv run --offline --no-project \
  .agents/skills/rf-literature-review/scripts/y_factor_references.py \
  | /tmp/rf-y-factor-compare
rustfmt --edition 2021 --check \
  .agents/skills/rf-literature-review/scripts/compare_y_factor.rs
```

The probe calls the actual public API for the selected candidate. Its gate
rejects any NaN, finite/nonfinite mismatch, or wrong-sign infinity, and requires
at most one ULP distance when both values are finite. Disposable probe copies
that replaced the expected-positive-infinity sample's selected result with NaN
or negative infinity both failed the classification gate; production files were
not changed. These checks apply to the supplied sample, not the whole domain.
The generator
uses standard-library `Fraction` throughout, followed by correctly rounded
rational-to-float conversion, avoiding any finite-precision oracle cancellation
or need to tune precision. For the downstream NF reference, Decimal at 80 and
160 digits stabilized to the same f64 value:

```bash
uv run --offline --no-project - <<'PY'
from decimal import Decimal as D, localcontext
for precision in (80, 160):
    with localcontext() as context:
        context.prec = precision
        print(precision, float(10 * (1 + D(136) / 290).log10()))
PY
```

Verification also ran `cargo fmt --all -- --check`, warnings-denied all-target
all-feature Clippy, `cargo test --all-features`, and warnings-denied rustdoc.
The clean-checkout `CARGO_NET_OFFLINE=true just check` gate additionally verifies
packaging. These checks establish software behavior, not instrument accuracy.
