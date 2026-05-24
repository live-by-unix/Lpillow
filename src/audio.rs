use minimp3::{Decoder, Frame};
use std::fs::File;
use std::io::BufWriter;

pub struct PCMData {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

pub fn decode_mp3(path: &str) -> Result<PCMData, Box<dyn std::error::Error>> {
    let file = File::open(path)?;

    let mut decoder = Decoder::new(file);

    let mut samples = Vec::new();

    let mut sample_rate = 44100u32;

    while let Ok(Frame {
        data,
        sample_rate: sr,
        channels,
        ..
    }) = decoder.next_frame()
    {
        sample_rate = sr as u32;

        if channels == 1 {
            for sample in data {
                samples.push(sample as f32 / i16::MAX as f32);
            }
        } else {
            let mut i = 0usize;

            while i + 1 < data.len() {
                let left = data[i] as f32;
                let right = data[i + 1] as f32;

                samples.push(((left + right) * 0.5) / i16::MAX as f32);

                i += 2;
            }
        }
    }

    Ok(PCMData {
        samples,
        sample_rate,
    })
}

pub fn encode_wav(
    path: &str,
    samples: &[f32],
    sample_rate: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let writer = BufWriter::new(File::create(path)?);

    let mut wav = hound::WavWriter::new(writer, spec)?;

    for sample in samples {
        let s = (sample * i16::MAX as f32)
            .clamp(i16::MIN as f32, i16::MAX as f32) as i16;

        wav.write_sample(s)?;
    }

    wav.finalize()?;

    Ok(())
}
