# Windows build status

The manual workflow `.github/workflows/windows-installer.yml` builds the
Tauri desktop installer on a Windows runner and uploads the `*-setup.exe`
as a workflow artifact. In GitHub Actions, open **Windows installer (manual)**,
select **Run workflow**, then download the artifact from the finished run.
The source archive must first be placed in a GitHub repository; unpack its
contents at the repository root. A private repository is fine.

This archive fixes the Python script packaging bug: Tauri now bundles the
`python/` directory as resources and resolves it through the Tauri resource
directory instead of assuming an unpacked source tree. The Rust package
version is also synchronized with the app version.

**The installer has not been compiled or tested on Windows.** The current
execution environment is Linux and has no Rust toolchain or Windows runner.
More importantly, this source does not yet provide a working standalone
transcription install:

- The Tauri GUI expects `~/.audio2sheets/venv/Scripts/python.exe`, but no GUI
  command creates or populates it. The CLI has separate setup code that the
  GUI does not invoke.
- `python/requirements.txt` omits `transkun`, which `python/pipeline.py`
  imports. A dependency declaration and a smoke test against the actual
  Transkun API are still necessary.
- PM2S weights and YourMT3 code/checkpoints are not bundled. The CLI has
  setup logic for PM2S, but the GUI never runs it.

The uploaded workflow builds a Windows **GUI installer**, not an offline
ready-to-transcribe program. Do not treat an installer artifact as proof that
the audio-to-MIDI pipeline works. A standalone build needs a Windows Python
runtime, compatible ML dependencies, models, and actual end-to-end tests.
