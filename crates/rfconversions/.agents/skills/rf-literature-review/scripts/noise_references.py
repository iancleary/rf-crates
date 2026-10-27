"""Emit exact-f64-input Decimal references; stdlib only, run via uv offline."""
import math
import struct
from decimal import Decimal as D, localcontext


def reference(kind, value, precision):
    with localcontext() as ctx:
        ctx.prec = precision
        x = D.from_float(value)
        if kind == "nf":
            result = D(290) * ((x * D(10).ln() / 10).exp() - 1)
        else:
            ratio = 1 + x / 290
            if ratio < 0:
                return math.nan
            if ratio == 0:
                return -math.inf
            result = 10 * ratio.ln() / D(10).ln()
        return float(result)


nf = [0.0, -0.0, 0.5, 3.0, 6.0, 1e-16, -1e-16,
      math.ulp(0.0), -math.ulp(0.0), 1e-320, -1e-320,
      math.ulp(1.0), -math.ulp(1.0), 3057.9, 3057.922,
      3057.923, math.nextafter(3057.923175620178, 0.0),
      3057.923175620178, math.nextafter(3057.923175620178, math.inf),
      4000.0, -4000.0]
te = [0.0, -0.0, 35.0, 290.0, 864.510794605142, 1e-16, -1e-16,
      math.ulp(0.0), -math.ulp(0.0), 1e-320, -1e-320, 2e-322, -2e-322,
      290 * math.ulp(1.0), -290 * math.ulp(1.0),
      math.nextafter(-290.0, 0.0), -290.0,
      math.nextafter(-290.0, -math.inf), -145.0,
      math.nextafter(-145.0, -math.inf), float.fromhex('0x1.fffffffffffffp+1023')]
for kind, values in (("nf", nf), ("te", te)):
    for x in values:
        a, b = reference(kind, x, 500), reference(kind, x, 800)
        assert a == b or math.isnan(a) and math.isnan(b)
        bits = struct.unpack('>Q', struct.pack('>d', x))[0]
        print(f"{kind},{bits:016x},{b!r}")
