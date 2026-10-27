"""Independent Decimal references for compare_power.rs; standard library only."""

from decimal import Decimal as D, localcontext

CASES = [
    (0.0, 0.0, 1e-14),
    (0.0, -160.0, 1e-29),
    (4000.0, 4000.0, 1e-12),
    (-4000.0, -4000.0, 1e-12),
    (-100.0, -110.0, 3e-14),
    (-3.010299956639812, -3.010299956639812, 1e-15),
    (1000.0, -4000.0, 1e-12),
]


def references(precision):
    with localcontext() as ctx:
        ctx.prec = precision
        return [
            float(10 * (D(10) ** (D.from_float(a) / 10)
                        + D(10) ** (D.from_float(b) / 10)).log10())
            for a, b, _ in CASES
        ]


low_precision = references(160)
high_precision = references(240)
assert low_precision == high_precision, "Increase oracle precision"
for (a, b, tolerance), expected in zip(CASES, high_precision):
    print(f"{a},{b},{expected!r},{tolerance}")
