use std::fs;

pub fn run(
    input: &str,
    output: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read(input)?;

    let ext = crate::utils::extension(input);

    let file_type = crate::utils::detect_type(&bytes, &ext);

    if file_type != "wav"
        && file_type != "flac"
        && file_type != "mp3"
    {
        return Err("unsupported input format".into());
    }

    let sha256 = crate::utils::sha256_hex(&bytes);

    let ltf = crate::ltf::LTFFile {
        file_type,
        file_length: bytes.len(),
        sha256,
        bytes,
    };

    let text = crate::ltf::encode(&ltf);

    fs::write(output, text)?;

    println!("compressed {} -> {}", input, output);

    Ok(())
}
