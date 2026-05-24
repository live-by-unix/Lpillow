use sha2::{Digest, Sha256};

pub fn extension(path: &str) -> String {
    match std::path::Path::new(path).extension() {
        Some(v) => v.to_string_lossy().to_lowercase(),
        None => String::new(),
    }
}

pub fn detect_type(bytes: &[u8], ext: &str) -> String {
    if bytes.len() >= 12 && &bytes[0..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
        return "wav".to_string();
    }

    if bytes.len() >= 4 && &bytes[0..4] == b"fLaC" {
        return "flac".to_string();
    }

    if bytes.len() >= 3 && &bytes[0..3] == b"ID3" {
        return "mp3".to_string();
    }

    if ext == "wav" || ext == "flac" || ext == "mp3" {
        return ext.to_string();
    }

    "unknown".to_string()
}

pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();

    hasher.update(data);

    let result = hasher.finalize();

    hex::encode(result)
}

pub fn chunk_string(data: &str, size: usize) -> Vec<String> {
    let mut out = Vec::new();

    let mut offset = 0usize;

    while offset < data.len() {
        let end = (offset + size).min(data.len());

        out.push(data[offset..end].to_string());

        offset = end;
    }

    out
}
