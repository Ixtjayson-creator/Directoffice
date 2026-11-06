use tokio::process::Command;
use std::path::PathBuf;
use anyhow::Result;

/// Executes Whisper speech‑to‑text on a local audio file.
/// Returns the transcript on success.
pub async fn transcribe(audio_path: PathBuf) -> Result<String> {
    // Placeholder: invoke the bundled whisper.cpp binary.
    // In a real implementation, the binary would be located in `AI/Runtime/`.
    let output = Command::new("whisper_cpp")
        .arg("-m")
        .arg("AI/Models/Speech/whisper.tiny.bin")
        .arg("-f")
        .arg(audio_path)
        .output()
        .await?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("Whisper failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

/// Executes the local language model (llama.cpp) to summarize text.
/// Returns the summary on success.
pub async fn summarize(text: &str) -> Result<String> {
    // Placeholder: invoke the bundled llama.cpp binary.
    let output = Command::new("llama_cpp")
        .arg("-m")
        .arg("AI/Models/Language/qwen3.gguf")
        .arg("-p")
        .arg(text)
        .output()
        .await?;
    if !output.status.success() {
        return Err(anyhow::anyhow!("Llama failed: {}", String::from_utf8_lossy(&output.stderr)));
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

