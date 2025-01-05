# SVG to PNG Converter

A Rust-based tool for converting SVG files to PNG format with support for Chinese text rendering.

## Features

- Convert SVG files to PNG format
- Support for Chinese text rendering with various fallback fonts
- Maintain aspect ratio while resizing
- Batch conversion capability
- Configurable output dimensions

## Requirements

- Rust (latest stable version)
- Cargo package manager

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/svg_2_img.git
cd svg_2_img
```

2. Build the project:
```bash
cargo build --release
```

## Usage

### Single File Conversion

Convert a single SVG file to PNG:

```bash
cargo run -- -i input.svg -o output.png -w 768 -t 1024
```

Parameters:
- `-i, --input`: Input SVG file path
- `-o, --output`: Output PNG file path
- `-w, --width`: Output width in pixels (optional)
- `-t, --height`: Output height in pixels (optional)

### Batch Conversion

Use the provided script to convert multiple SVG files:

```bash
./convert_all.sh
```

The script will:
1. Create a `png` subdirectory in the input directory
2. Convert all SVG files with the pattern `*-svg.svg`
3. Save PNG files with dimensions 768x1024

## Font Support

The converter includes support for various Chinese fonts:
- SimSun
- SimKai
- KaiTi
- Microsoft YaHei
- WenQuanYi Micro Hei
- Noto Sans CJK (SC, TC, JP)

## Dependencies

- resvg: SVG rendering library
- tiny-skia: 2D graphics library
- usvg: SVG parser
- clap: Command-line argument parser
