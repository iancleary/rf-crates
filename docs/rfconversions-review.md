# rfconversions PR review

## Scope and disposition

Reviewed the four open PRs against their common main base
`061c86863d61eac08d6c01ca148bdb428f31dc1c`, including production formulas,
public-API tests, numerical reference generators, comparison programs, and
operating limits. This was a source and local-execution review, not an assertion
of independent reviewers or a new full literature survey.

| PR | Decision | Reason |
| --- | --- | --- |
| [36: literature proposals](https://github.com/iancleary/rfconversions/pull/36) | Integrate | Keeps sources, assumptions, retained comparisons, and limitations discoverable without adding runtime dependencies. |
| [37: direct NF/temperature conversions](https://github.com/iancleary/rfconversions/pull/37) | Integrate | Direct elementary functions avoid cancellation; subnormal scaling and the negative-temperature boundary have independent references. Existing factor APIs stay unchanged. |
| [38: independent power addition](https://github.com/iancleary/rfconversions/pull/38) | Integrate | Explicit same-reference scalar API, NaN/infinity contracts, shifted logarithmic evaluation, and a real noise/interference example. |
| [39: Y-factor receiver temperature](https://github.com/iancleary/rfconversions/pull/39) | Integrate | Validates the measurement domain, retains negative diagnostic temperatures, and checks scaled fused arithmetic against exact rational inputs. |

No blocking production defect was found within the documented contracts. The
PRs were integrated into this local monorepo with their commit histories intact.
Their upstream GitHub PR states were not changed. Tests passed after each local
integration, not only after the combined tree was assembled.

## Review qualifications

- `add_power_db` is broader than an exclusively dBm-named helper. Both inputs
  must share a power reference. It is neither coherent field addition nor gain
  addition. Keep that contract when adopting it in gainlineup.
- The power-addition helper does not preserve every subnormal correction or
  guarantee correct rounding around a cancelling near-zero answer. Those limits
  are documented rather than concealed by permissive tests.
- The NF/temperature comparison program is diagnostic: printing a result is not
  a pass/fail assertion. The focused public integration tests supply its automated
  acceptance gate. The new retained runner executes both.
- The Y-factor result becomes ill-conditioned near Y=1. Numerical stability
  cannot correct measurement error, gain drift, or mismatch. The documented
  extreme overflow-boundary limitation is outside the sampled accuracy claim.
- Existing MSRV declarations were preserved, but their older compilers were not
  separately executed. Current-toolchain results do not certify Rust 1.70.
- One workspace adoption correction was needed in gainlineup's Touchstone
  frequency accessor; it is a dependency-version migration, not a change to the
  reviewed rfconversions formulas.

## Repeatable numerical evidence

Run from the workspace root:

```bash
CARGO_NET_OFFLINE=true just numerics
```

The maintained runner lives beside the existing comparison programs and
reference generators in the rfconversions literature-review skill. It discovers
Cargo's actual target directory instead of assuming a per-crate `target/`, builds
the real library, and compiles the existing comparisons against that artifact.
No extra Python dependencies or external services are involved.

The retained methods are:

- NF/temperature: exact binary64 inputs converted to Decimal; rounded references
  must agree at 500 and 800 decimal digits. Public integration tests cover small
  inputs, algebraic boundaries, ordinary points, and selected extreme values.
- Power addition: unshifted Decimal formula at 160 and 240 digits. The production
  helper passes the sampled bounds while linear conversion and ordinary shifted
  logarithms demonstrate their distinct failures.
- Y-factor: exact `Fraction` arithmetic for 4,011 selected/seeded input triples.
  The comparison checks nonfinite classifications and finite ULP distance.

Local execution reproduced the documented selected-candidate results: zero
failed power-addition samples and zero Y-factor nonfinite classification
mismatches, with at most one ULP observed in that finite sample. These are sampled
correctness observations, not performance results or full-domain guarantees.

For equations, references, tolerances, and limitations, use the owning guides:

- [Direct noise conversion accuracy](../crates/rfconversions/docs/noise-conversion-accuracy.md)
- [Independent power addition](../crates/rfconversions/docs/power-addition.md)
- [Y-factor calibration](../crates/rfconversions/docs/y-factor-calibration.md)
- [Original literature review](../crates/rfconversions/docs/literature-review.md)
