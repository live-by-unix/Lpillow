# LPillow 1.0.0

LPillow is a CAM (Computer Audio Model) audio transformer.

LPillow converts MP3 audio into a fully textual spectral representation called LTF (LPillow Text Format), then reconstructs audio back from the text representation.

## Features

- MP3 decoding
- FFT spectral transforms
- Fully textual frame storage
- Deterministic reconstruction
- Offline operation
- Stable Rust
- Single binary
- No unsafe Rust

## Commands

Compress audio into LTF:

./lpillow compress input.mp3 -o output.ltf

Reconstruct audio from LTF:

./lpillow unpress output.ltf -o restored.wav

## LTF Format

One frame per line:

LP{bin1,bin2,bin3,...;amp;phase;noise}

Each frequency bin stores real and imaginary spectral data.

## Build

cargo build --release

## Install Binary

cp target/release/lpillow /usr/local/bin/lpillow

## Test Audio

https://filesamples.com/samples/audio/mp3/sample3.mp3
