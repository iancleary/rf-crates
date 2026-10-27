# Literature-backed scalar conversion improvements

## Status and mental model

This page records the original proposals against baseline `061c868` (v0.7.7
crate manifest). Runtime source and public API are unchanged on this research
branch. The three implementations are now in **independent, unmerged PRs targeting
main**; accept any subset without merging this research PR first. The final
implementations include boundary handling beyond the sketches below.

| Idea | Implementation PR | Selected approach and comparison evidence |
| --- | --- | --- |
| Direct NF↔Te | [#37](https://github.com/iancleary/rfconversions/pull/37) | Range-aware direct formulas; [three-formula comparison](https://github.com/iancleary/rfconversions/blob/fix/stable-noise-temperature/docs/noise-conversion-accuracy.md) |
| Independent power sum | [#38](https://github.com/iancleary/rfconversions/pull/38) | Max-shifted `ln_1p`; [three-formula comparison](https://github.com/iancleary/rfconversions/blob/feat/log-domain-power-addition/docs/power-addition.md) |
| Y-factor→Te | [#39](https://github.com/iancleary/rfconversions/pull/39) | Scaled fused numerator with overflow fallback; [six-formula comparison](https://github.com/iancleary/rfconversions/blob/feat/y-factor-temperature/docs/y-factor-calibration.md) |

Each branch passed `just check` independently, including packaging. A disposable
checkout with all three final patches also passed; every pair of branch tips
merged without conflicts. An initial README conflict was removed by separating
the additions. A fresh read-only review found no production blockers and prompted
explicit extreme-range limitations plus a stricter nonfinite comparison check.
These checks used Rust 1.99; the declared Rust 1.70 minimum was not separately run.
No PR has been merged or released as part of this work.

The repeatable procedure and original numerical probe live in the
[RF literature-review skill](../.agents/skills/rf-literature-review/SKILL.md).
Use each implementation guide for its final candidate comparisons and known
limits; the baseline observations and sketches below remain historical evidence.

A scalar library has two different opportunities: compute an existing formula
more faithfully, or supply a missing formula with an unambiguous convention.
Neither requires a system simulator. The best first change here is numerical:
NF↔temperature currently loses small differences by going through a noise
factor close to one. Two additive helpers could then serve independent-power
aggregation and receiver calibration without owning hardware chains or instruments.

| Priority | Candidate | Caller benefit | Cost and principal risk |
| --- | --- | --- | --- |
| 1 | Stable direct NF↔Te conversions | Preserve small nonzero results without an API change | Small implementation; rounding compatibility and boundary verification |
| 2 | Add two independent powers in dB | Avoid repeated linear conversions and their range limits | Small additive API; callers must not confuse coherent signals with independent powers |
| 3 | Y-factor to equivalent receiver temperature | Normalize hot/cold calibration results before existing NF conversion | Small formula; measurement assumptions and ill-conditioning dominate |

This is a targeted review, not a systematic literature survey. No speedup is
claimed; neither benchmark throughput nor calibration hardware was measured.

## Sources and access depth

1. **David Goldberg (1991)**, “What Every Computer Scientist Should Know About
   Floating-Point Arithmetic,” *ACM Computing Surveys* 23(1), 5–48,
   [DOI: 10.1145/103162.103163](https://doi.org/10.1145/103162.103163).
   Read the cancellation discussion and Theorem 4 in the
   [authorized Oracle reprint](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html#719).
   It explains why forming `1 + x` before a logarithm destroys small `x`.
   The RF substitutions below are our application of that principle, not formulas
   claimed to appear in Goldberg. The reprint's later addendum is not the 1991 paper.
2. **Pierre Blanchard, Desmond J. Higham, Nicholas J. Higham (2021)**,
   “Accurately Computing the Log-Sum-Exp and Softmax Functions,” *IMA Journal of
   Numerical Analysis* 41(4), 2311–2330,
   [DOI: 10.1093/imanum/draa038](https://doi.org/10.1093/imanum/draa038),
   first published online in 2020. Read the
   [institutional manuscript](https://eprints.maths.manchester.ac.uk/2765/3/paper_nomarks.pdf),
   version May 15, 2020, physical PDF pages 2–3 and 10–11: equation (1.3),
   equation (4.1), and Algorithm 4.1. The cover labels the report MIMS 2019.16;
   that is not the journal publication year. The inspected shifted formulation
   uses the largest input and `log1p`; our two-power, base-10 formula is a
   specialization, not an RF result or a copied implementation from that paper.
3. **James J. Condon and Scott M. Ransom (2016)**, *Essential Radio Astronomy*,
   Princeton University Press, ISBN 9781400881161,
   [DOI: 10.1515/9781400881161](https://doi.org/10.1515/9781400881161).
   Read the [NRAO online chapter, §3.6.6](https://www.cv.nrao.edu/~sransom/web/Ch3.html#S6.SS6):
   equations (3.165)–(3.168) give the hot/cold Y-factor relation;
   (3.169)–(3.171) give F, Te, NF, and the 290 K reference. Equations were
   inspected in the page's MathML/TeX, not inferred from an abstract. Citations
   below use this online edition's numbering; no print-page equivalence is assumed.

Author/title/identifier metadata were cross-checked against Crossref; the second
source's inspected manuscript is available through the University of Manchester.
An attempted Keysight 5952-8255 PDF retrieval returned HTML and failed extraction;
it is **not evidence** for any recommendation. No paid sources, copied reference
code, or paper payloads are included in the repository.

## 1. Preserve small NF↔temperature differences

**Owner:** [`src/noise.rs`](../src/noise.rs),
`noise_temperature_from_noise_figure` and `noise_figure_from_noise_temperature`.

The current paths compute `F = 10^(NF/10)` then `290*(F-1)`, and
`F = 1 + Te/290` then `10*log10(F)`. When the intermediate F rounds to exactly
one, the requested nonzero difference is gone. Converting back cannot recover it.
This is a numerical gap, not a different noise reference convention.

Keep both signatures and the crate's `constants::T0`. Candidate direct forms:

```text
Te = T0 * exp_m1(NF * (ln(10)/10))       # kelvin from dB
NF = (10/ln(10)) * ln_1p(Te/T0)          # dB from kelvin
```

These follow from source 3's RF definitions and source 1's cancellation analysis.
Rust already supplies both elementary functions; no numerical dependency is needed.
A consumer converting a very low-noise component's NF to an equivalent temperature
gets a nonzero value instead of a numerical zero. The tiny samples below are
stress cases, not a claim that instruments resolve noise figures of `1e-16 dB`.

**Compatibility contract to verify:** physical Te is nonnegative, but preserve the
existing algebraic extension rather than adding a new panic or clamping negatives.
Te = -290 K gives -infinity dB; Te < -290 K gives NaN. Positive infinities remain
positive infinity, NaN propagates, and NF = -infinity tends to -290 K. Check signed
zero explicitly: direct `ln_1p`/`exp_m1` may retain a negative zero where the current
composition returns positive zero. Decide and document that observable difference.
Some existing tests assert exact nontrivial transcendental results; assess changes
against independent tolerances, not the old rounding accident. Do not blindly
replace the factor APIs: they must still return an F rounded to `f64`.

**Acceptance:** independently rounded reference points below; ordinary NF values
0, 0.5, 3, 6 dB; Te 0, 35, 290 K; both signs near zero; neighbors of -290 K;
NaN/infinities; subnormals; large finite inputs near output overflow. Use relative
error away from zero and explicit absolute or classification checks at zero and
singularities. A round trip is supplemental, not the oracle. The sampled probe is
not proof of accuracy across the full `f64` domain.

## 2. Add independent powers without leaving the log domain

**Owner:** [`src/power.rs`](../src/power.rs). Proposed new signature:
`add_power_db(a: f64, b: f64) -> f64` (name subject to API review).

Current callers must compose `linear_to_db(db_to_linear(a) + db_to_linear(b))`.
That is mathematically right but can lose a small contribution or produce infinity
when the final log-domain result is representable. For finite inputs, specialize
source 2, equation (4.1), with `m=max(a,b)` and `n=min(a,b)`:

```text
sum_db = m + (10/ln(10)) * ln_1p(10^((n-m)/10))
```

Both inputs must share a power reference: dBm+dBm yields dBm, dBW+dBW yields dBW,
or two power-ratio dB values relative to the same base yield that reference's dB.
Example: two independent 0 dBm noise contributions total 3.010299956639812 dBm.
This adds powers, not their logarithms. It does **not** combine coherent voltage
phasors, imply matched impedance from amplitude dB, or average readings in dB.
A two-argument scalar helper avoids introducing a chain model or collection API.

**Proposed boundary policy:** NaN takes precedence and propagates; otherwise any
+infinity yields +infinity. Negative infinity represents zero power: adding it
returns the other input, and two negative infinities return negative infinity.
Handle these explicitly before subtraction; `inf-inf` is not a usable shift.
The prototype below exercises finite inputs only and does not implement this policy.

**Acceptance:** equal powers at 0 and ±4000 dB, 0 plus -160 dB, swapped operands,
one zero-power operand, both zero-power operands, NaN in either position including
with infinity, and the positive-infinity cases. Use an absolute dB error bound
near a zero-dB output. A small term may still be lost when the *final result*
cannot represent it; this algorithm prevents avoidable intermediate loss, not
all rounding. No performance advantage is established. This is additive API work
with low implementation cost but a significant documentation obligation.

## 3. Convert a hot/cold Y-factor to receiver temperature

**Owner:** [`src/noise.rs`](../src/noise.rs). Proposed new signature:
`noise_temperature_from_y_factor(y: f64, hot_kelvin: f64, cold_kelvin: f64) -> f64`.

Source 3, equations (3.167)–(3.168), defines a **linear detected power ratio**
`Y = Phot/Pcold = (Th + Te)/(Tc + Te)`. The text's detector voltages are proportional
to power; Y is not an RF voltage-amplitude ratio. If measurements arrive as two dBm
readings, use `power::db_to_linear(hot_dbm - cold_dbm)` first. Then compute:

```text
Te = (Th - Y*Tc)/(Y - 1) = (Th - Tc)/(Y - 1) - Tc
```

The right-hand form avoids the `Y*Tc` intermediate; it does not cure the physical
conditioning problem. Feed the kelvin result into the existing Te→NF helper with
T0=290 K. For Y=2, Th=290 K, Tc=77 K, the result is **136 K**, not 290 K.
This is a useful scalar handoff for a calibration caller, not an instrument driver,
ENR correction pipeline, or uncertainty-propagation framework.

**Assumptions and proposed policy:** finite Th>Tc≥0 and finite Y>1; return NaN for
invalid inputs, including infinities and Y=1. Require unchanged linear receiver
gain and bandwidth between measurements, calibrated effective load temperatures
at the input reference plane, and the classical thermal-noise regime. Source loss,
mismatch, gain drift, detector nonlinearity, or inappropriate quantum-temperature
assumptions can invalidate the inferred result. Keep negative computed Te as a
visible diagnostic rather than silently clamp it: for Tc>0, Y>Th/Tc indicates
inconsistency with a nonnegative receiver temperature under this model.

**Acceptance:** the 136 K analytic point; Th=290, Tc=0, Y=2 gives 290 K;
Th=300, Tc=100, Y=3 gives exactly zero; that pair at Y=5 gives -50 K diagnostic;
invalid temperature ordering and Y boundaries return NaN. Near Y=1, use a
high-precision oracle but also document sensitivity:
`dTe/dY = -(Th-Tc)/(Y-1)^2`. At Th=290, Tc=77, Y=1.01, a Y error of 0.0001
produces about 213 K of first-order temperature error. No rearrangement removes
that measurement limitation. API usefulness should be confirmed with a real
calibration consumer before implementation.

## Reproduce the evidence

Run the [skill's offline Rust probe](../.agents/skills/rf-literature-review/SKILL.md#rerun-the-numerical-evidence).
It compiles the actual library, calls its public API, prints candidate outputs,
and compares them with literal independent references. Observed at the baseline:

| Conversion | Current API or composition | Independent reference (rounded) |
| --- | --- | --- |
| NF=1e-16 dB → Te | 0 K | 6.677496769682733e-15 K |
| Te=1e-16 K → NF | 0 dB | 1.4975671789767304e-18 dB |
| Add 0 and -160 dB | 0 dB | 4.342944819032518e-16 dB |
| Add 4000 and 4000 dB | +infinity | 4003.01029995664 dB |
| Add -4000 and -4000 dB | -infinity | -3996.98970004336 dB |
| Y=2, Th=290 K, Tc=77 K | No existing API | 136 K |

The candidates matched the probe's references within its `2e-14` relative
screening tolerance, including ordinary RF points. This is an experiment, not a
promised libm error bound or a new production test suite. Extreme dB values isolate
floating-point range behavior; they are not realistic transmitter powers.

Reference literals came from Python's standard-library Decimal at 80-digit
precision, using decimal inputs rather than the candidate's binary elementary
functions. Regenerate the difficult points without installing packages:

```bash
uv run --offline --no-project --python 3.13 - <<'PY'
from decimal import Decimal as D, localcontext
with localcontext() as ctx:
    ctx.prec = 80
    print('NF to Te:', D(290) * ((D('1e-16') * D(10).ln()/10).exp() - 1))
    print('Te to NF:', 10 * (1 + D('1e-16')/290).ln()/D(10).ln())
    print('Power sum:', 10 * (1 + D('1e-16')).log10())
    for level in ('0', '4000', '-4000'):
        print('Equal powers:', D(level) + 10 * D(2).log10())
PY
```

The command requires an already available Python 3.13; do not install one
implicitly. For exhaustive work, generate references from the exact binary inputs
as well, and increase oracle precision until expected rounded values stabilize.
Neither these Decimal calculations nor the probe validate calibration hardware.

## Production sequence and deferred scope

1. Review any subset of the independent PRs above. Their Rustdoc, README summaries,
   runnable examples, and integration coverage accompany the implementation.
   Merging one does not require the others.
2. For #37, review changed rounding and signed-zero behavior. For #38, review the
   common-reference and independent-power contract. For #39, review the invalid
   input/negative diagnostic policy and confirm the calibration assumptions for
   the intended consumer; no instrument was tested.
3. Use the implementation guides' acceptance evidence, not this page's original
   sketch probe, to judge the final algorithms. The power helper can still lose
   extreme subnormal corrections; Y-factor rounding can shift the overflow
   boundary. Neither promises correctly rounded results over all `f64` inputs.
4. Re-run `just check` after any requested changes. Keep the branches independent
   and repeat combined-patch verification if their shared documentation or APIs
   change. Update the status links when individual PRs merge.

A passive-loss/physical-temperature helper is another plausible gap (there is
already a comment in `src/noise.rs`), but this review did not verify a primary
source for its full matched-network assumptions, so it is deferred. Friis cascade
is already implemented; restating it is not a new feature. S-parameter mismatch
correction belongs in `touchstone`; ordered calibration or receiver chains belong
in their owning higher-level tool. Quantum noise models and generic unit types
would expand the contract substantially and are not recommended in this slice.
