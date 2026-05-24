# LPillow — The World’s First Lossless CAM

LPillow is a **0%‑loss Computer Audio Model (CAM)** that transforms audio files into a deterministic text format (LTF) and reconstructs them **bit‑for‑bit identical**.

Not “lossless audio.”  
Not “sounds identical.”  
**Lossless reality.**

LPillow preserves every byte of the original file — headers, metadata, padding, container structure, everything.

To verify:

```bash
sha256sum input.wav restored.wav
```

If the hashes match, LPillow reconstructed the file exactly.

---

## Features

- **0% loss** — perfect byte‑for‑byte reconstruction  
- **Deterministic LTF format** (LPillow Text Format)  
- **Supports WAV, FLAC, MP3**  
- **Preserves metadata, padding, and container structure**  
- **No re‑encoding or transcoding**  
- **No drift, no mutation, no entropy loss**  
- **Cross‑platform Rust CLI**

---

## Usage

### Compress

```bash
lpillow compress input.wav -o song.ltf
lpillow compress input.flac -o song.ltf
lpillow compress input.mp3 -o song.ltf
```

### Unpress

```bash
lpillow unpress song.ltf -o restored.wav
lpillow unpress song.ltf -o restored.flac
lpillow unpress song.ltf -o restored.mp3
```

### Verify 0% Loss

```bash
sha256sum input.wav restored.wav
```

---

## What Is LTF?

LTF (LPillow Text Format) is a reversible, line‑based text representation of the original audio file.

It stores:

- file type  
- file length  
- raw bytes (hex or base64)

Nothing is changed.  
Nothing is approximated.  
Nothing is lost.

---

## Why LPillow?

Traditional audio formats preserve *sound*.  
LPillow preserves **the file itself**.

This makes LPillow the first true **Lossless CAM** — a reversible representation of audio containers, metadata, and binary structure.

---

## Build

```bash
cargo build --release
```

Binary output:

```
target/release/lpillow
```

--- 

## Project Structure

```
lpillow/
 ├── Cargo.toml
 ├── README.md
 ├── .gitignore
 └── src/
     ├── main.rs
     ├── cli.rs
     ├── compress.rs
     ├── unpress.rs
     ├── ltf.rs
     ├── utils.rs
     ├── wav.rs
     ├── flac.rs
     └── mp3.rs
```

---
## **SECURITY**
**PLEASE STAY UPDATE ON YOUR VERSION OF LPILLOW.** 
**IF YOU HAVE FOUND A SECURITY ISSUE OF ANY SORT, PLEASE OPEN A GITHUB ISSUE ON THIS REPO.**
**THANK YOU.**

**ESPAÑOL O ESPAÑOLA**
**Por favor, mantenga actualizada su versión de LPILLOW.**
**Si has encontrado algún problema de seguridad de cualquier tipo, por favor, abre un issue en GitHub en este repositorio.**     
**Gracias.**
--- 
## License

BSD-3.0 License.
