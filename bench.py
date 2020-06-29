#!/usr/bin/env python3
import timeit

HOWMANY = 10
python_setup = """
import struct
from math import sin, pi
to_bytes = lambda vals: struct.pack(str(len(vals)) + "d", *vals)
VOLUME = 0.5
SAMPLES = 48000
DURATION = 60 / 136
STANDARD_PITCH = 440.0
ATTACK = 0.0004
"""
python_load = f"""
n_entries = DURATION * SAMPLES
step = (STANDARD_PITCH * 2 * pi) / SAMPLES
data = [VOLUME * sin(step * n) for n in range(0, int(n_entries))] * {HOWMANY}
with open("pyoutput.bin", "wb") as handle:
    handle.write(to_bytes(data))
"""
rust_setup = "import waverly"
rust_load = f"""
waverly.song_pure("output.bin", {HOWMANY})
"""

print("Benchmarking Pure python")
timeit.timeit(python_load, setup=python_setup)
print("Benchmarking Rust python")
timeit.timeit(rust_load, setup=rust_setup)
