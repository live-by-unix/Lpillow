use num_complex::Complex32;
use rustfft::FftPlanner;

pub fn forward(frame: &[f32]) -> Vec<Complex32> {
    let mut planner = FftPlanner::<f32>::new();

    let fft = planner.plan_fft_forward(frame.len());

    let mut buffer: Vec<Complex32> = frame
        .iter()
        .map(|v| Complex32::new(*v, 0.0))
        .collect();

    fft.process(&mut buffer);

    buffer
}

pub fn inverse(freq: &[Complex32]) -> Vec<f32> {
    let mut planner = FftPlanner::<f32>::new();

    let fft = planner.plan_fft_inverse(freq.len());

    let mut buffer = freq.to_vec();

    fft.process(&mut buffer);

    buffer
        .iter()
        .map(|c| c.re / freq.len() as f32)
        .collect()
}
