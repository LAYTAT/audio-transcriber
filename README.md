# Audio Transcriber

Local audio transcription using Whisper. Supports Chinese and English.

## Usage

```bash
# Convert audio to WAV first
ffmpeg -i audio.m4a -ar 16000 -ac 1 audio.wav

# Transcribe Chinese
cargo run --release -- audio.wav zh

# Transcribe English
cargo run --release -- audio.wav en
```

## Output

- Prints transcription to console
- Saves to `<filename>_transcription.txt`

## First Run

Downloads Whisper model (~3GB) on first run.
