# rust-ocr

A command-line tool for extracting text from images and PDF files.

## Download a release

Prebuilt binaries are available on the
[GitHub Releases page](https://github.com/akadotsh/rust-cli-app/releases).

The current prebuilt binary supports **macOS on Apple Silicon (arm64)**. It is
dynamically linked to Homebrew libraries and is not compatible with Intel Macs,
Linux, or Windows.

Install the required libraries:

```bash
brew install tesseract libarchive
```

Download `rust-ocr-v0.1.0-macos-arm64.tar.gz` from the Releases page, then
extract and install it:

```bash
tar -xzf rust-ocr-v0.1.0-macos-arm64.tar.gz
chmod +x rust-ocr
sudo install -m 755 rust-ocr /usr/local/bin/rust-ocr
```

Confirm that the installation works:

```bash
rust-ocr --version
```

## Usage

Pass an image or PDF path to the command:

```bash
rust-ocr path/to/image.png
rust-ocr path/to/document.pdf
```

Image files are processed with Tesseract OCR. PDF files are read for embedded
text; scanned PDFs without an embedded text layer are not currently OCRed.

Run `rust-ocr --help` to see the command-line help.

## Build from source

Install [Rust](https://www.rust-lang.org/tools/install) and the native
dependencies:

```bash
brew install tesseract leptonica libarchive
```

Clone and build the project:

```bash
git clone https://github.com/akadotsh/rust-cli-app.git
cd rust-cli-app
cargo build --release
```

The compiled binary will be available at `target/release/rust-ocr`.
