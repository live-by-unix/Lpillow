# LPillow 1.0

LPillow 1.0 is the world's first 0% loss CAM (Computer Audio Model).

LPillow transforms audio files into a deterministic text-based LTF representation and reconstructs the original file BIT-FOR-BIT IDENTICAL with ZERO loss.

Supported formats:

- WAV
- FLAC
- MP3

LPillow operates at the FILE level.

LPillow preserves:

- all headers
- all metadata
- all frames
- all chunks
- all padding
- all extension data
- all raw bytes

0% loss means:

sha256(input) == sha256(output)

for all supported formats.

Compress WAV:

lpillow compress input.wav -o song.ltf

Compress FLAC:

lpillow compress input.flac -o song.ltf

Compress MP3:

lpillow compress input.mp3 -o song.ltf

Unpress WAV:

lpillow unpress song.ltf -o restored.wav

Unpress FLAC:

lpillow unpress song.ltf -o restored.flac

Unpress MP3:

lpillow unpress song.ltf -o restored.mp3

LTF format example:

LP1|type=wav|length=123456
SHA256|abcdef...
DATA|0011223344...

LPillow reconstructs the original file exactly.
