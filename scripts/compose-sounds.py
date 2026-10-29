"""Reproducible original Lotse sound family. Python stdlib; no samples/services."""
import math
import struct
import wave
from pathlib import Path

RATE = 44100
OUT = Path(__file__).resolve().parents[1] / 'ui/src/audio'
# D major pentatonic: soft felt-like partials, airy stereo echoes, no ticking.
SCORES = {
    'hint': (2.8, [(0, 587.33, .44, -.25), (.24, 880, .30, .28)]),
    'start': (1.9, [(0, 440, .30, -.18), (.16, 587.33, .32, .18)]),
    'complete': (4.4, [(0, 587.33, .38, -.35), (.28, 739.99, .32, .25), (.58, 880, .30, -.12), (.92, 1174.66, .20, .35)]),
}
for name, (duration, notes) in SCORES.items():
    channels = [[0.] * int(RATE * duration) for _ in range(2)]
    for onset, hz, amp, pan in notes:
        for echo, gain in [(0, 1), (.19, .12), (.37, .065)]:
            start = int((onset + echo) * RATE)
            for i in range(start, len(channels[0])):
                t = (i - start) / RATE
                envelope = (1 - math.exp(-t / .035)) ** 2 * math.exp(-t / .65)
                value = sum(weight * math.sin(2 * math.pi * hz * partial * t) * math.exp(-t * decay)
                            for partial, weight, decay in [(1, 1, 0), (2, .16, 1.8), (3, .045, 3)])
                for ch, balance in enumerate([math.sqrt((1-pan)/2), math.sqrt((1+pan)/2)]):
                    channels[ch][i] += value * envelope * amp * gain * balance
    peak = max(abs(x) for c in channels for x in c)
    scale = .25 / max(peak, .25)  # quiet consistent ceiling, no limiting distortion
    pcm = bytearray()
    for i, (left, right) in enumerate(zip(*channels)):
        fade = min(1, (len(channels[0]) - 1 - i) / (RATE * .18))
        pcm.extend(struct.pack('<hh', int(left*fade*scale*32767), int(right*fade*scale*32767)))
    with wave.open(str(OUT / f'{name}.wav'), 'wb') as out:
        out.setparams((2, 2, RATE, 0, 'NONE', 'not compressed'))
        out.writeframes(pcm)
