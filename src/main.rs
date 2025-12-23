use kalosm::sound::*;
use rodio::Decoder;
use std::{env, fs};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let audio_path = args.get(1).expect("Usage: transcribe <audio.wav> [lang]\n  lang: zh (Chinese), en (English, default)");
    let lang = args.get(2).map(|s| s.as_str()).unwrap_or("en");

    let language = match lang {
        "zh" => WhisperLanguage::Chinese,
        _ => WhisperLanguage::English,
    };

    println!("📥 Loading Whisper model...");
    let model = WhisperBuilder::default()
        .with_source(WhisperSource::large_v3_turbo())
        .build()
        .await?;
    println!("✅ Model loaded!");

    println!("🎵 Transcribing: {} ({})", audio_path, lang);
    let contents = fs::read(audio_path)?;
    let audio = Decoder::new(std::io::Cursor::new(contents))?;

    let mut text = model.transcribe(audio).with_language(language);
    let mut full_text = String::new();

    while let Some(segment) = text.next().await {
        for chunk in segment.chunks() {
            print!("{chunk}");
            full_text.push_str(&chunk.to_string());
        }
    }
    println!();

    let output_path = format!("{}_transcription.txt", audio_path.trim_end_matches(".wav"));
    fs::write(&output_path, &full_text)?;
    println!("\n💾 Saved to: {}", output_path);

    Ok(())
}
