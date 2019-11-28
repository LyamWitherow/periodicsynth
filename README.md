# PeriodicSynth
[![Build Status](https://travis-ci.org/nullvideo/periodicsynth.svg?branch=master)](https://travis-ci.org/nullvideo/periodicsynth)

A basic periodic waveform synthesizer for generating the most common types of waveforms that a signal generator is able to generate. Basically, it mimics the capabilities of a signal generator.

---

```bash
# Add the line below to the "Cargo.toml"
periodicsynth = "0.1.3"
```

```rust
/**
 * A basic usage of the library, a triangle wave
 * with a frequency of 440hZ and sample rate of
 * 44.1khZ and bit-depth of 64bits.
 */
use periodicsynth;

fn main() {
    /* Intialise an vector with 0 values.
       The length of the vector is the sample rate. */
    let mut samples = vec![0f64; 44100];

    /* Synthesize a triangle wave with frequency of 440hZ. */
    periodicsynth::synth(&mut samples, periodicsynth::PerodicFunction::Sine, 440.0);
}
```


## Motivation
The WebAudio API's [`OscillatorNode`](https://developer.mozilla.org/en-US/docs/Web/API/OscillatorNode) generates periodic waveforms on demand with several types of periodic function types with arbitrary frequency and sample rate.
