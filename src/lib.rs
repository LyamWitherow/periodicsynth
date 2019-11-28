use std::f64::consts::PI;

/// List of periodic functions can be synthesised,
/// also includes a Null synthesizer for generating
/// signals with `0` values.
pub enum PeriodicFunction {
    Sine,
    Square,
    Triangle,
    Sawtooth,
    /// This method is equavalient to intialising a vector with `0` values.
    Null,
}

/// Synthesizes a periodic waveform with a arbitary frequency and samplerate.
/// Note that, frequency should be less than half the samplerate (as of the nyquist theorem).
///
/// #### Functions implemented:
/// - Sine
/// - Square
/// - Triangle
/// - Sawtooth
///
/// **Note:** the size of the output vector is considered the samplerate,
/// the output vector must be resized before synthesizing.
///
/// ## Arguments
/// * `samples` &mdash; Vector to render oscillated samples into.
/// * `pfunc` &mdash; Periodic function used to oscillate.
/// * `freq` &mdash; Frequency of the function to oscillate on.
pub fn synth(samples: &mut Vec<f64>, pfunc: PeriodicFunction, freq: f64) {
    let sample_rate = samples.len();

    match pfunc {
        PeriodicFunction::Sine => {
            for (i, s) in samples.iter_mut().enumerate() {
                let p = i as f64 / sample_rate as f64;
                *s = ((2.0 * PI * freq) / p).sin();
            }
        },
        PeriodicFunction::Square => {
            for (i, s) in samples.iter_mut().enumerate() {
                let p = i as f64 / sample_rate as f64;
                *s = ((2.0 * PI * freq) / p).sin().signum();
            }
        },
        PeriodicFunction::Triangle => {
            for (i, s) in samples.iter_mut().enumerate() {
                let p = i as f64 / sample_rate as f64;
                *s = 4.0/p * (freq - p/2.0 * (2.0*freq / p + 0.5).floor());
            }
        },
        PeriodicFunction::Sawtooth => {
            for (i, s) in samples.iter_mut().enumerate() {
                let p = i as f64 / sample_rate as f64;
                *s = 2.0 * (freq/p - (0.5 + freq/p).floor());
            }
        },
        PeriodicFunction::Null => {
            for s in samples.iter_mut() {
                *s = 0.0;
            }
        }
    }
}
