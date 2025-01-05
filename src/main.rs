use std::path::PathBuf;
use clap::Parser;
use resvg::tiny_skia;
use resvg::usvg::{self, TreeParsing, fontdb, TreeTextToPath};

#[derive(Parser)]
#[command(author, version, about = "Convert SVG to PNG with custom size")]
struct Args {
    /// Input SVG file path
    #[arg(short, long)]
    input: PathBuf,

    /// Output PNG file path
    #[arg(short, long)]
    output: PathBuf,

    /// Output width in pixels (optional)
    #[arg(short = 'w', long)]
    width: Option<u32>,

    /// Output height in pixels (optional)
    #[arg(short = 't', long = "height")]
    height: Option<u32>,
}

fn convert_svg_to_png(
    svg_path: &PathBuf,
    png_path: &PathBuf,
    target_width: Option<u32>,
    target_height: Option<u32>
) -> Result<(), Box<dyn std::error::Error>> {
    // Read SVG file
    let svg_data = std::fs::read(svg_path)?;
    
    // Create font database and load system fonts
    let mut fontdb = fontdb::Database::new();
    fontdb.load_system_fonts();
    
    // Add fallback fonts for Chinese characters
    let fallback_fonts = vec![
        "SimSun", "SimKai", "KaiTi", "Microsoft YaHei", "WenQuanYi Micro Hei",
        "Noto Sans CJK SC", "Noto Sans CJK TC", "Noto Sans CJK JP",
    ];
    
    for font in fallback_fonts {
        fontdb.set_serif_family(font);
        fontdb.set_sans_serif_family(font);
    }

    // Create options
    let opt = usvg::Options {
        resources_dir: std::path::Path::new(svg_path).parent().map(|p| p.to_path_buf()),
        font_family: "Arial".to_string(),
        font_size: 12.0,
        languages: vec!["zh".to_string(), "en".to_string()],
        ..Default::default()
    };
    
    // Create a new tree with font database
    let mut tree = usvg::Tree::from_data(&svg_data, &opt)?;
    tree.convert_text(&fontdb);
    
    // Get the original size of the SVG
    let original_size = tree.view_box.rect;
    
    // Calculate the target size
    let (width, height) = match (target_width, target_height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let aspect_ratio = original_size.height() / original_size.width();
            (w, (w as f32 * aspect_ratio) as u32)
        },
        (None, Some(h)) => {
            let aspect_ratio = original_size.width() / original_size.height();
            ((h as f32 * aspect_ratio) as u32, h)
        },
        (None, None) => (
            original_size.width() as u32,
            original_size.height() as u32
        ),
    };
    
    // Create a pixel map with the target size
    let mut pixmap = tiny_skia::Pixmap::new(width, height)
        .ok_or("Failed to create pixel map")?;
    
    // Calculate the transform to fit the SVG into the target size
    let scale_x = width as f32 / original_size.width();
    let scale_y = height as f32 / original_size.height();
    
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);
    
    // Create resvg renderer and render
    let rtree = resvg::Tree::from_usvg(&tree);
    rtree.render(transform, &mut pixmap.as_mut());
    
    // Save the result as PNG
    pixmap.save_png(png_path)?;
    
    Ok(())
}

fn main() {
    let args = Args::parse();

    match convert_svg_to_png(&args.input, &args.output, args.width, args.height) {
        Ok(_) => println!("Successfully converted SVG to PNG!"),
        Err(e) => eprintln!("Error converting SVG: {}", e),
    }
}
