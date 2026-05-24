pub const FRAME_SIZE: usize = 1024;

pub fn split_frames(samples: &[f32], frame_size: usize) -> Vec<Vec<f32>> {
    let mut frames = Vec::new();

    let mut index = 0usize;

    while index < samples.len() {
        let mut frame = vec![0.0f32; frame_size];

        let end = (index + frame_size).min(samples.len());

        let slice = &samples[index..end];

        for (i, sample) in slice.iter().enumerate() {
            frame[i] = *sample;
        }

        frames.push(frame);

        index += frame_size;
    }

    frames
}

pub fn normalize(samples: &[f32]) -> Vec<f32> {
    let mut peak = 0.0f32;

    for sample in samples {
        let value = sample.abs();

        if value > peak {
            peak = value;
        }
    }

    if peak == 0.0 {
        return samples.to_vec();
    }

    samples.iter().map(|v| *v / peak).collect()
}
