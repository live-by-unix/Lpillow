use crate::audio;
use crate::fft;
use crate::ltf;
use crate::utils;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(input)?;

    let reader = BufReader::new(file);

    let mut pcm = Vec::new();

    let mut sample_rate = 44100u32;

    let mut count = 0usize;

    for line in reader.lines() {
        let line = line?;

        if line.starts_with("LPHEADER{") {
            let body = line
                .strip_prefix("LPHEADER{")
                .and_then(|v| v.strip_suffix('}'))
                .ok_or("invalid header")?;

            for token in body.split(';') {
                if let Some(value) = token.strip_prefix("sample_rate=") {
                    sample_rate = value.parse::<u32>()?;
                }
            }

            println!("detected_sample_rate={}", sample_rate);

            continue;
        }

        let (bins, _, _, _) = ltf::decode(&line)?;

        let frame = fft::inverse(&bins);

        pcm.extend(frame);

        count += 1;

        if count % 64 == 0 {
            println!("decoded_frame={}", count);
        }
    }

    println!("reconstructed_samples={}", pcm.len());

    let normalized = utils::normalize(&pcm);

    audio::encode_wav(output, &normalized, sample_rate)?;

    Ok(())
}
