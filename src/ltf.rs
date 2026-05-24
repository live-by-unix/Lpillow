pub struct LTFFile {
    pub file_type: String,
    pub file_length: usize,
    pub sha256: String,
    pub bytes: Vec<u8>,
}

pub fn encode(file: &LTFFile) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "LP1|type={}|length={}\n",
        file.file_type,
        file.file_length
    ));

    out.push_str(&format!(
        "SHA256|{}\n",
        file.sha256
    ));

    let hex_data = hex::encode(&file.bytes);

    let chunks = crate::utils::chunk_string(&hex_data, 128);

    for chunk in chunks {
        out.push_str("DATA|");
        out.push_str(&chunk);
        out.push('\n');
    }

    out
}

pub fn decode(
    text: &str,
) -> Result<LTFFile, Box<dyn std::error::Error>> {
    let mut file_type = String::new();

    let mut file_length = 0usize;

    let mut sha256 = String::new();

    let mut hex_data = String::new();

    for line in text.lines() {
        if line.starts_with("LP1|") {
            for field in line.split('|').skip(1) {
                let parts: Vec<&str> = field.split('=').collect();

                if parts.len() != 2 {
                    continue;
                }

                match parts[0] {
                    "type" => file_type = parts[1].to_string(),
                    "length" => file_length = parts[1].parse()?,
                    _ => {}
                }
            }
        } else if line.starts_with("SHA256|") {
            sha256 = line.replacen("SHA256|", "", 1);
        } else if line.starts_with("DATA|") {
            hex_data.push_str(&line.replacen("DATA|", "", 1));
        }
    }

    let bytes = hex::decode(&hex_data)?;

    Ok(LTFFile {
        file_type,
        file_length,
        sha256,
        bytes,
    })
}
