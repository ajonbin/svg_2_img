# SVG to Image Converter

A Rust-based tool for converting SVG files to PNG format with customizable dimensions.

## Features
- Convert SVG files to PNG format
- Maintain aspect ratio when only width or height is specified
- Support custom output dimensions
- High-quality rendering using resvg and tiny-skia

## Dependencies
- resvg (0.35.0): SVG rendering
- tiny-skia (0.10.0): Rasterization
- usvg (0.35.0): SVG parsing
- clap (4.4.11): Command-line argument parsing

## Installation
```bash
cargo build --release
```

## Usage
The program supports the following command-line arguments:
```bash
Options:
  -i, --input <INPUT>    Input SVG file path (required)
  -o, --output <OUTPUT>  Output PNG file path (required)
  -w, --width <WIDTH>    Output width in pixels (optional)
  -t, --height <HEIGHT>  Output height in pixels (optional)
  -h, --help            Print help
  -V, --version         Print version
```

### Examples
1. Convert with original SVG dimensions:
```bash
cargo run -- -i input.svg -o output.png
```

2. Convert with specific width (height auto-calculated to maintain aspect ratio):
```bash
cargo run -- -i input.svg -o output.png -w 1080
```

3. Convert with specific height (width auto-calculated to maintain aspect ratio):
```bash
cargo run -- -i input.svg -o output.png -t 1920
```

4. Convert with both dimensions specified:
```bash
cargo run -- -i input.svg -o output.png -w 1080 -t 1920
```

## Note
- When only width or height is specified, the other dimension will be automatically calculated to maintain the original aspect ratio
- The output will always be in PNG format
- If neither width nor height is specified, the original SVG dimensions will be used
