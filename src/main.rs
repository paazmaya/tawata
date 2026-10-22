use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod common;
mod djvu_converter;
mod epub_converter;
mod mobi_converter;
mod resolver;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Converts Ebooks (EPUB, MOBI, DJVU) to Markdown."
)]
struct Cli {
    /// Input ebook file, glob pattern (e.g. "*.epub"), or directory
    #[arg(required = true)]
    input: String,

    #[arg(short, long)]
    recursive: bool,

    #[arg(short, long)]
    autocorrect: bool,

    #[arg(long, default_value_t = true)]
    oxfmt: bool,
}

/// Dispatches an ebook file to its appropriate format-specific converter.
pub fn convert_ebook_file(path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("epub") => epub_converter::process_epub(path, autocorrect, oxfmt),
        Some("mobi") => mobi_converter::process_mobi(path, autocorrect, oxfmt),
        Some("djvu") | Some("djv") => djvu_converter::process_djvu(path, autocorrect, oxfmt),
        Some(unsupported) => {
            eprintln!(
                "Skipping {:?}: Extension '{}' is not currently supported.",
                path, unsupported
            );
            Ok(())
        }
        None => {
            eprintln!("Skipping {:?}: No file extension found.", path);
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    let paths = resolver::resolve_inputs(&cli.input, cli.recursive)?;

    if paths.is_empty() {
        println!("No Ebook files found for input: {}", cli.input);
        return Ok(());
    }

    println!("Found {} Ebook files to process.", paths.len());

    for path in paths {
        println!("\nProcessing: {:?}", path);

        if let Err(e) = convert_ebook_file(&path, cli.autocorrect, cli.oxfmt) {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_ebook_file_unsupported() {
        let unsupported_path = Path::new("test.pdf");
        assert!(convert_ebook_file(unsupported_path, false, false).is_ok());

        let no_ext_path = Path::new("no_extension_file");
        assert!(convert_ebook_file(no_ext_path, false, false).is_ok());
    }
}
