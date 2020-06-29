#!/usr/bin/env python3
import struct
from math import sin, pi

HOWMANY = 10000
VOLUME = 0.5
SAMPLES = 48000
DURATION = 60 / 136
STANDARD_PITCH = 440.0
ATTACK = 0.0004


to_bytes = lambda vals: struct.pack(f"{len(vals)}d", *vals)
n_entries = DURATION * SAMPLES
step = (STANDARD_PITCH * 2 * pi) / SAMPLES
data = [VOLUME * sin(step * n) for n in range(0, int(n_entries))] * HOWMANY
with open("pyoutput.bin", "wb") as handle:
    handle.write(to_bytes(data))

# ffplay -f f64be -ar 48000 pyoutput.bin -showmode 1
