use anyhow::Result;
use clap::Parser;
use std::path::Path;

mod boko_converter;
mod common;
mod djvu_converter;
mod resolver;

/// Command line arguments of `tawata`.
#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Converts Ebooks (EPUB, MOBI, AZW3, KFX, DJVU) to Markdown."
)]
struct Cli {
    /// Input ebook file, glob pattern (e.g. "*.epub"), or directory
    #[arg(required = true)]
    input: String,

    /// Search directories recursively
    #[arg(short, long)]
    recursive: bool,

    /// Apply CJK/Western spacing fixes and merge split words
    #[arg(short, long)]
    autocorrect: bool,

    /// Skip formatting the output with the OXC markdown formatter
    #[arg(long)]
    no_oxfmt: bool,

    /// Write the whole book to a single book.md instead of one file per chapter
    /// (does not apply to DJVU)
    #[arg(long)]
    single_file: bool,
}

/// Dispatches an ebook file to its appropriate format-specific converter.
pub fn convert_ebook_file(
    path: &Path,
    autocorrect: bool,
    oxfmt: bool,
    single_file: bool,
) -> Result<()> {
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|s| s.to_ascii_lowercase());

    match ext.as_deref() {
        Some("epub") | Some("mobi") | Some("azw3") | Some("azw") | Some("kfx") => {
            boko_converter::process_boko(path, autocorrect, oxfmt, single_file)
        }
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

/// Parses arguments, resolves the inputs and converts each file, reporting per-file errors.
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

        if let Err(e) = convert_ebook_file(&path, cli.autocorrect, !cli.no_oxfmt, cli.single_file) {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parses_all_flags() {
        let cli = Cli::parse_from([
            "tawata",
            "-r",
            "-a",
            "--single-file",
            "--no-oxfmt",
            "./library",
        ]);
        assert_eq!(cli.input, "./library");
        assert!(cli.recursive && cli.autocorrect && cli.single_file && cli.no_oxfmt);

        let defaults = Cli::parse_from(["tawata", "book.epub"]);
        assert!(
            !defaults.recursive
                && !defaults.autocorrect
                && !defaults.single_file
                && !defaults.no_oxfmt
        );
    }

    #[test]
    fn test_convert_ebook_file_dispatch_errors_on_missing_file() {
        for name in ["missing.epub", "missing.azw3", "missing.djvu"] {
            assert!(convert_ebook_file(Path::new(name), false, false, false).is_err());
        }
    }

    #[test]
    fn test_convert_ebook_file_dispatches_djvu() {
        let mut data = b"AT&TFORM".to_vec();
        data.extend_from_slice(&22u32.to_be_bytes());
        data.extend_from_slice(b"DJVUINFO");
        data.extend_from_slice(&10u32.to_be_bytes());
        data.extend_from_slice(&[0, 10, 0, 10, 24, 0, 100, 0, 22, 0]);
        let dir = std::env::temp_dir().join(format!("tawata_main_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("d.djvu");
        std::fs::write(&path, data).unwrap();
        assert!(convert_ebook_file(&path, false, false, false).is_ok());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_convert_ebook_file_unsupported() {
        let unsupported_path = Path::new("test.pdf");
        assert!(convert_ebook_file(unsupported_path, false, false, false).is_ok());

        let no_ext_path = Path::new("no_extension_file");
        assert!(convert_ebook_file(no_ext_path, false, false, false).is_ok());
    }
}
