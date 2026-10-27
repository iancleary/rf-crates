"""Emit exact-binary-input Y-factor references; standard library only."""
from fractions import Fraction
import math
import random
import struct


def bits(x):
    return struct.unpack(">Q", struct.pack(">d", x))[0]


def reference(y, hot, cold):
    # Fraction eliminates precision selection and cancellation in the oracle.
    result = (Fraction(hot) - Fraction(y) * Fraction(cold)) / (Fraction(y) - 1)
    try:
        return float(result)
    except OverflowError:
        return math.inf if result > 0 else -math.inf


cases = [
    (2., 290., 77.), (2., 290., 0.), (3., 300., 100.), (5., 300., 100.),
    (math.nextafter(1., 2.), 290., 77.),
    (1.5, 1.5e308, 5e307),
    (1e308, 300., 100.),
    (math.nextafter(3., 4.), 300., 100.),
    (math.nextafter(2., 1.), 1., .5),
    (1.5, 2 * math.ulp(0.), math.ulp(0.)),
    (math.nextafter(1., 2.), math.nextafter(1., 2.), 1.),
    (math.nextafter(1., 2.), 1.7e308, 0.),
]
rng = random.Random(165168)
for _ in range(3000):
    hot = math.ldexp(rng.uniform(1., 2.), rng.randrange(-1074, 1023))
    cold = hot * rng.random()
    if not hot > cold:
        continue
    y = rng.choice([math.nextafter(1., 2.), rng.uniform(1., 5.),
                    math.ldexp(rng.uniform(1., 2.), rng.randrange(1, 1023))])
    cases.append((y, hot, cold))
# Add cancellation points with rounded input ratios, not ideal decimal inputs.
for _ in range(1000):
    cold = math.ldexp(rng.uniform(1., 2.), rng.randrange(-1070, 1020))
    hot = cold * rng.uniform(1.01, 8.)
    if math.isfinite(hot) and hot > cold > 0:
        y = hot / cold
        cases.append((y, hot, cold))
for y, hot, cold in cases:
    print(*(f"{bits(x):016x}" for x in (y, hot, cold, reference(y, hot, cold))))
