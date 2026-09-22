# Muse 0.1.1 — Windows x64 build

[Download the CI artifact](https://github.com/galinagavnoedina-cmyk/test/actions/runs/35782013231/artifacts/10719516398).
Unzip it and run `Muse_0.1.1_x64-setup.exe`. The installer is unsigned.

SHA-256 of the installer:
`58EA1332F32010C064E092AFE4122FDD8D6116D560CD0DBD4D50030EA29031A6`.

This build bundles an isolated Python 3.11 runtime, Transkun 2.0.1 and its
model, and miniaudio decoding. No system Python, Colab, Hugging Face, or
ffmpeg installation is needed for the **Solo Piano** mode.

The original GUI's Multi-Instrument / YourMT3 mode was hidden because its
model and code are absent from the source archive. The Windows build returns
the raw Transkun MIDI in both `.mid` and `.perf.mid`: physical key release
timings and pedal CC events are preserved. Hand splitting, quantized score,
Demucs stem separation, and MusicXML export are not included in this build.
It works best with a recording containing only piano.

Windows CI built the installer and used the bundled runtime to transcribe
a synthetic WAV into MIDI. The GUI has not been interactively tested on the
user's Windows computer, and no real piano recording has been benchmarked.
The CI artifact expires on 2026-12-21; the branch source retains the build
workflow.
