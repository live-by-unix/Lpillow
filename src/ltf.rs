use num_complex::Complex32;

pub fn encode(
    bins: &[Complex32],
    amp: f32,
    phase: f32,
    noise: f32,
) -> String {
    let mut out = String::new();

    out.push_str("LP{");

    for (i, bin) in bins.iter().enumerate() {
        out.push_str(&format!("{:.10}:{:.10}", bin.re, bin.im));

        if i + 1 != bins.len() {
            out.push(',');
        }
    }

    out.push(';');

    out.push_str(&format!("{:.10}", amp));

    out.push(';');

    out.push_str(&format!("{:.10}", phase));

    out.push(';');

    out.push_str(&format!("{:.10}", noise));

    out.push('}');

    out
}

pub fn decode(
    line: &str,
) -> Result<(Vec<Complex32>, f32, f32, f32), Box<dyn std::error::Error>> {
    let body = line
        .strip_prefix("LP{")
        .and_then(|v| v.strip_suffix('}'))
        .ok_or("invalid ltf format")?;

    let parts: Vec<&str> = body.split(';').collect();

    if parts.len() != 4 {
        return Err("invalid ltf sections".into());
    }

    let mut bins = Vec::new();

    for token in parts[0].split(',') {
        let pair: Vec<&str> = token.split(':').collect();

        if pair.len() != 2 {
            return Err("invalid fft bin".into());
        }

        let re: f32 = pair[0].parse()?;
        let im: f32 = pair[1].parse()?;

        bins.push(Complex32::new(re, im));
    }

    let amp: f32 = parts[1].parse()?;
    let phase: f32 = parts[2].parse()?;
    let noise: f32 = parts[3].parse()?;

    Ok((bins, amp, phase, noise))
}
