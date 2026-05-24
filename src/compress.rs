use crate::audio;
use crate::fft;
use crate::ltf;
use crate::utils;
use std::fs::File;
use std::io::{BufWriter, Write};

pub fn run(input: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pcm = audio::decode_mp3(input)?;

    println!("sample_rate={}", pcm.sample_rate);
    println!("samples={}", pcm.samples.len());

    let frames = utils::split_frames(&pcm.samples, utils::FRAME_SIZE);

    let file = File::create(output)?;

    let mut writer = BufWriter::new(file);

    writer.write_all(format!("LPHEADER{{sample_rate={}}}\n", pcm.sample_rate).as_bytes())?;

    for (index, frame) in frames.iter().enumerate() {
        let fft_bins = fft::forward(frame);

        let amp = frame.iter().map(|v| v.abs()).sum::<f32>() / frame.len() as f32;

        let phase = fft_bins.first().map(|c| c.arg()).unwrap_or(0.0);

        let noise = 0.0f32;

        let token = ltf::encode(&fft_bins, amp, phase, noise);

        writer.write_all(token.as_bytes())?;
        writer.write_all(b"\n")?;

        if index % 64 == 0 {
            println!("encoded_frame={}", index);
        }
    }

    writer.flush()?;

    Ok(())
}
