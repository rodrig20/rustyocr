mod screenshot;
use tempfile::NamedTempFile;
use leptess::LepTess;
use wl_clipboard_rs::copy::{MimeType, Options, Source};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Set language
    let args: Vec<String> = env::args().collect();
    let lang = if args.len() > 1 {
        &args[1]
    } else {
        "eng"
    };

    // Save screenshot temporarily
    let temp_file = NamedTempFile::new()?;
    screenshot::save(temp_file.path().to_str().unwrap());

    // Config leptess (Tesseract)
    let mut lt = LepTess::new(Some("/usr/share/tessdata"), lang)?;
    lt.set_image(temp_file.path().to_str().unwrap())?;

    // Copy to clipboard
    let text = lt.get_utf8_text()?;
    let opts = Options::new();
    opts.copy(Source::Bytes(text.to_string().into_bytes().into()), MimeType::Text)?;

    // Show
    println!("{text}");

    Ok(())
}
