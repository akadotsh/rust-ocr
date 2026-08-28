use clap::Parser;
use std::path::Path;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    path: PathBuf,
}

mod parse;

fn main() {
    let cli = Cli::parse();

    let path = Path::new(&cli.path);
    if let Some(extension) = path.extension() {
        if extension == "pdf" {
            parse::extract_pdf_text(&cli.path);
        } else {
            parse::extract_image_text(path);
        }
    }
}
