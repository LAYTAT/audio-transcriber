# Audio Transcriber

Local audio transcription using Whisper. Supports Chinese and English.

## Prerequisites

1. **Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **ffmpeg**
   ```bash
   # macOS
   brew install ffmpeg
   
   # Ubuntu/Debian
   sudo apt install ffmpeg
   ```

## Installation

```bash
git clone https://github.com/LAYTAT/audio-transcriber.git
cd audio-transcriber
cargo build --release
```

## Usage

```bash
# Convert audio to WAV first (required format)
ffmpeg -i audio.m4a -ar 16000 -ac 1 audio.wav

# Transcribe Chinese
cargo run --release -- audio.wav zh

# Transcribe English
cargo run --release -- audio.wav en
```

## Output

- Prints transcription to console in real-time
- Saves to `<filename>_transcription.txt`

## Notes

- First run downloads Whisper model (~3GB)
- Supports WAV format (convert other formats with ffmpeg)
- Uses `whisper-large-v3-turbo` model for best multilingual results
