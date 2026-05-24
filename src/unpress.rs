use std::fs;

pub fn run(
    input: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = fs::read_to_string(input)?;

    let ltf = crate::ltf::decode(&text)?;

    if ltf.bytes.len() != ltf.file_length {
        return Err("file length mismatch".into());
    }

    let actual_sha256 = crate::utils::sha256_hex(&ltf.bytes);

    if actual_sha256 != ltf.sha256 {
        return Err("sha256 mismatch".into());
    }

    fs::write(output, &ltf.bytes)?;

    println!("unpressed {} -> {}", input, output);

    Ok(())
}
