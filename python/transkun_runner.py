"""Run the installed Transkun V2 model while preserving key release and CC64.

The upstream command line reader forces MP3 input. miniaudio lets this app
decode WAV, MP3, FLAC and OGG without requiring a system ffmpeg installation.
"""

import importlib.resources

import miniaudio
import moduleconf
import numpy as np
import torch
from transkun.Data import writeMidi


def run_transkun(input_path, output_path):
    model_files = importlib.resources.files("transkun.pretrained")
    with importlib.resources.as_file(model_files.joinpath("2.0.conf")) as conf_path:
        config = moduleconf.parseFromFile(str(conf_path))
    with importlib.resources.as_file(model_files.joinpath("2.0.pt")) as weight_path:
        # These weights ship with the pinned pip package, not with user input.
        checkpoint = torch.load(str(weight_path), map_location="cpu", weights_only=False)

    model = config["Model"].module.TransKun(conf=config["Model"].config)
    state = checkpoint.get("best_state_dict", checkpoint.get("state_dict"))
    model.load_state_dict(state, strict=False)
    model.eval()

    decoded = miniaudio.decode_file(
        input_path,
        output_format=miniaudio.SampleFormat.FLOAT32,
        nchannels=1,
        sample_rate=model.fs,
    )
    samples = np.frombuffer(decoded.samples, dtype=np.float32).copy().reshape(-1, 1)
    with torch.inference_mode():
        events = model.transcribe(torch.from_numpy(samples), discardSecondHalf=False)

    # Transkun's writeMidi emits note events plus pedal control changes. It
    # does not extend physical key holds to the end of the sustain pedal.
    writeMidi(events).write(output_path)
