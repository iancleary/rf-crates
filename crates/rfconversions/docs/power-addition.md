# Adding independent powers in dB

## Reference first, then addition

[`power::add_power_db`](../src/power.rs) takes two `f64` power levels and
returns their independent (incoherent) power sum in the same reference:

- dBm with dBm returns dBm (reference: 1 mW).
- dBW with dBW returns dBW (reference: 1 W).
- Power-ratio dB values must use the same base power.

For example, two independent 0 dBm contributions are each 1 mW, so their sum
is 2 mW, or approximately 3.010299956639812 dBm. Adding the *numbers* in dB
would instead return 0 dBm, which is not the power sum. Likewise, averaging
dB readings is not averaging power. Gains in a cascade add in dB, but that is
a different operation from adding independent powers.

Normalize mixed references before calling. For receiver noise at -100 dBm
and independent interference at -140 dBW, both measured at the same reference
plane and bandwidth:

```rust
use rfconversions::power::{add_power_db, dbw_to_dbm, dbm_to_watts};
let total_dbm = add_power_db(-100.0, dbw_to_dbm(-140.0));
let total_watts = dbm_to_watts(total_dbm);
assert!((total_watts - 1.1e-13).abs() < 1e-27);
```

This workflow is exercised in
[`tests/practical_scenarios.rs`](../tests/practical_scenarios.rs).
Coherent signals need phase-aware voltage/field addition, not this helper.
No impedance matching, decorrelation, or reference-plane normalization is
inferred from two bare numbers; the caller owns those assumptions.

## Calculation and boundary contract

Mathematically the result is `10*log10(10^(a/10) + 10^(b/10))`. Directly
forming those linear powers can overflow or underflow even when the answer
in dB is representable. With `high=max(a,b)` and `low=min(a,b)`, factor out
the larger power instead:

```text
high + (10/ln(10)) * ln_1p(10^((low-high)/10))
```

The exponent is nonpositive. `ln_1p(x)` evaluates `ln(1+x)` without first
rounding `1+x` to one for very small `x`. This preserves the correction for
0 plus -160 dB, where an ordinary `log10(1+x)` loses it.

Nonfinite inputs are handled **before** min/max and subtraction:

| Input condition (either order) | Result |
| --- | --- |
| Either input NaN, including alongside +infinity | NaN |
| Otherwise either input +infinity | +infinity |
| One input -infinity | Other input unchanged, including signed zero |
| Both inputs -infinity | -infinity |

Negative infinity means zero linear power; a finite 0 dB does not. NaN
payloads are not preserved as a contract. A finite signed zero otherwise
has the usual meaning of 0 dB, so adding two zero-dB powers is about 3.01 dB,
not signed-zero addition.

For `low=-f64::MAX` and `high=f64::MAX`, the finite subtraction overflows
to negative infinity. The exponent then contributes zero. This is harmless:
the smaller power is far below a representable correction to the larger
level. No special subtraction guard is needed. Even equal powers at
`±f64::MAX` or `±1e20` return the input level because the approximately 3 dB
increase is below half an output ULP. At 1000 and -4000 dB the smaller term
also disappears. These are floating-point stress cases, not physical powers.

Final addition can cancel when `high` is near -3.0103 dB. This simple
algorithm does not promise correct rounding or small *relative* error for
an output near zero. For the exact `f64` input pair
`(-3.010299956639812, -3.010299956639812)`, Decimal gives
`-1.3905958374036735e-16` dB; the observed helper gives
`-8.881784197001252e-16` dB. The absolute error is below `1e-15` dB.
That limitation is different from avoidably rounding away the tiny,
non-cancelling correction at `(0, -160)`.

There is also an extreme-subnormal limitation, distinct from final-result
rounding: an actual-API independent review found `add_power_db(0.0, -3238.0)`
returns zero. The exact-input Decimal reference is approximately
`6.883103678918900664602809690351e-324` dB, which rounds to the smallest positive
`f64` subnormal (`5e-324`). Scratch calculations at both 500 and 800 digits
agreed on that rounded reference. Here the `powf` term underflows to zero
*before* multiplication by `10/ln(10)`; `ln_1p` cannot recover it. Thus not all
representable corrections survive the intermediates. Independent review found
no ordinary RF regression, and universal correct rounding was not promised.
This observed approximation is documented as a limitation, not pinned as
mandatory test behavior.

## Alternatives and observed decision

The retained probe compares three implementations on the same inputs:

| Approach | Observation | Decision |
| --- | --- | --- |
| Existing `linear_to_db(db_to_linear(a) + db_to_linear(b))` | Returns 0 for `(0,-160)`, +infinity for equal 4000, -infinity for equal -4000 | Reject: avoidable intermediate loss/range failure |
| Max shift with `10*log10(1+small)` | Handles ±4000 but returns 0 for `(0,-160)` | Reject: loses the correction before the logarithm |
| Max shift with `ln_1p` (production API) | Passes the finite reference bounds and explicit boundary checks | Select: one scalar expression plus required nonfinite policy |

Observed on the local Rust toolchain, including swapped arguments: the first
two alternatives failed 6 and 2 of 14 finite samples respectively; production
failed none. Equal 0 dB inputs yielded `3.0102999566398112` dB in production,
versus the rounded reference `3.010299956639812` dB. Equal ±4000 inputs yielded
`4003.01029995664` and `-3996.98970004336` dB. The RF point `(-100,-110)`
yielded `-99.58607314841775` dB. The `(0,-160)` correction yielded
`4.3429448190325176e-16` dB versus `4.342944819032518e-16` dB.

Convergence here means passing these independent numerical and boundary
criteria with the simpler alternatives rejected for observed failures.
It is not a performance comparison, exhaustive `f64` proof, or universal
accuracy claim. Elementary-function rounding may vary across platforms.
The [focused public tests](../tests/power_addition.rs) use absolute dB bounds:
`1e-29` for the tiny positive correction, `1e-15` near a cancelling zero
output, and `1e-14` to `1e-12` for the other sampled magnitudes.

## Independent rationale and reference generation

The prior literature review established these numerical principles; the RF
specialization and this implementation are not copied reference code:

1. David Goldberg (1991), “What Every Computer Scientist Should Know About
   Floating-Point Arithmetic,” [DOI: 10.1145/103162.103163](https://doi.org/10.1145/103162.103163),
   cancellation discussion and Theorem 4 in the
   [Oracle reprint](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html#719):
   explains the loss from forming `1+x` before taking its logarithm.
2. Pierre Blanchard, Desmond J. Higham, Nicholas J. Higham (2021), “Accurately
   Computing the Log-Sum-Exp and Softmax Functions,”
   [DOI: 10.1093/imanum/draa038](https://doi.org/10.1093/imanum/draa038),
   [institutional manuscript](https://eprints.maths.manchester.ac.uk/2765/3/paper_nomarks.pdf),
   equation (4.1) and Algorithm 4.1: largest-input shifting and `log1p`.
   The manuscript is dated May 15, 2020; the journal volume is 2021.

The numerical oracle is independent of Rust's binary elementary functions:
[`power_references.py`](../.agents/skills/rf-literature-review/scripts/power_references.py)
uses standard-library Decimal and the *unshifted* mathematical sum. Inputs
are converted with `Decimal.from_float`, so the near-zero reference describes
the actual binary inputs rather than ideal decimal spellings. Rounded oracle
values must agree at 160 and 240 digits of precision. Extreme-finite cases
use the representability argument above rather than constructing impossible
Decimal exponents. No external packages or hardware measurements are involved.

## Rerun offline

From the repository root, with Rust, `uv`, and an already available Python
(no installations required):

```bash
set -o pipefail
CARGO_NET_OFFLINE=true cargo build --lib
rustfmt --edition 2021 --check .agents/skills/rf-literature-review/scripts/compare_power.rs
rustc --edition 2021 .agents/skills/rf-literature-review/scripts/compare_power.rs \
  --extern rfconversions=target/debug/librfconversions.rlib \
  -L dependency=target/debug/deps -o target/compare-power
uv run --offline --no-project \
  .agents/skills/rf-literature-review/scripts/power_references.py | target/compare-power
CARGO_NET_OFFLINE=true cargo test --test power_addition
```

The [probe](../.agents/skills/rf-literature-review/scripts/compare_power.rs)
calls the actual public API, checks both argument orders, prints all three
candidates, and asserts the decision criteria. It also exercises NaN precedence,
infinity policy, signed-zero identity, and extreme-finite rounding. Keep its
sibling Decimal generator with it when copying the experiment.

If offline execution cannot find Python or cached build dependencies, use an
already provisioned toolchain/cache; do not silently install anything. A custom
`CARGO_TARGET_DIR` requires adjusting the `rustc` paths above. The finite probe
is a small decision aid, not a general numerical testing framework. No collection
API, coherent-combining model, benchmark claim, or new dependency is introduced.
