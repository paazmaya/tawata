//! Command-line entry point and format dispatcher for `tawata`.

use anyhow::{Context, Result};
use clap::Parser;
use log::{debug, info, warn};
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
    about = "Converts EPUB, MOBI, AZW3, AZW, KFX, DJVU, and DJV ebooks to Markdown."
)]
struct Cli {
    /// Ebook file, glob pattern containing `*`, or directory. Supported formats:
    /// EPUB, MOBI, AZW3, AZW, KFX, DJVU, and DJV. Directories are scanned at the
    /// top level unless --recursive is used.
    #[arg(required = true)]
    input: String,

    /// Search directories and all their subdirectories for supported ebook files
    #[arg(short, long)]
    recursive: bool,

    /// Fix CJK/Western spacing and merge words split by spaces (may affect short phrases)
    #[arg(short, long)]
    autocorrect: bool,

    /// Skip OXC Markdown formatting (formatting is enabled by default)
    #[arg(long)]
    no_format: bool,

    /// Write supported ebook formats to book.md instead of one file per chapter;
    /// DJVU always produces one Markdown file per page
    #[arg(long)]
    single_file: bool,
}

/// Dispatches an ebook file to its appropriate format-specific converter.
///
/// Unsupported file extensions or files without extensions are skipped with a warning.
///
/// # Errors
///
/// Returns an error if converting a supported ebook file fails.
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

    debug!("Dispatching file {:?} (ext: {:?})", path, ext);

    match ext.as_deref() {
        Some("epub") | Some("mobi") | Some("azw3") | Some("azw") | Some("kfx") => {
            boko_converter::process_boko(path, autocorrect, oxfmt, single_file)
        }
        Some("djvu") | Some("djv") => djvu_converter::process_djvu(path, autocorrect, oxfmt),
        Some(unsupported) => {
            warn!(
                "Skipping {:?}: extension '{}' is not supported",
                path, unsupported
            );
            eprintln!(
                "Skipping {:?}: Extension '{}' is not currently supported.",
                path, unsupported
            );
            Ok(())
        }
        None => {
            warn!("Skipping {:?}: no file extension found", path);
            eprintln!("Skipping {:?}: No file extension found.", path);
            Ok(())
        }
    }
}

/// Resolves inputs from `cli` and converts each discovered ebook file, failing fast on errors.
fn run(cli: &Cli) -> Result<()> {
    let paths = resolver::resolve_inputs(&cli.input, cli.recursive)?;

    if paths.is_empty() {
        info!("No ebook files found for input: {}", cli.input);
        println!("No Ebook files found for input: {}", cli.input);
        return Ok(());
    }

    info!("Found {} ebook files to process", paths.len());
    println!("Found {} Ebook files to process.", paths.len());

    for path in paths {
        println!("\nProcessing: {:?}", path);
        convert_ebook_file(&path, cli.autocorrect, !cli.no_format, cli.single_file)
            .with_context(|| format!("Error processing {:?}", path))?;
    }

    Ok(())
}

/// Parses arguments, resolves the inputs, and converts each file, failing fast on conversion errors.
fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();
    debug!("Parsed CLI arguments: {:?}", cli);
    run(&cli)
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
            "--no-format",
            "./library",
        ]);
        assert_eq!(cli.input, "./library");
        assert!(cli.recursive && cli.autocorrect && cli.single_file && cli.no_format);

        let defaults = Cli::parse_from(["tawata", "book.epub"]);
        assert!(
            !defaults.recursive
                && !defaults.autocorrect
                && !defaults.single_file
                && !defaults.no_format
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

    #[test]
    fn test_run_with_empty_glob_and_valid_djvu() {
        let dir = std::env::temp_dir().join(format!("tawata_run_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        let empty_cli = Cli::parse_from(["tawata", dir.to_str().unwrap()]);
        assert!(run(&empty_cli).is_ok());

        let mut data = b"AT&TFORM".to_vec();
        data.extend_from_slice(&22u32.to_be_bytes());
        data.extend_from_slice(b"DJVUINFO");
        data.extend_from_slice(&10u32.to_be_bytes());
        data.extend_from_slice(&[0, 10, 0, 10, 24, 0, 100, 0, 22, 0]);
        let djvu_path = dir.join("doc.djvu");
        std::fs::write(&djvu_path, data).unwrap();

        let run_cli = Cli::parse_from(["tawata", "--no-format", djvu_path.to_str().unwrap()]);
        assert!(run(&run_cli).is_ok());

        let _ = std::fs::remove_dir_all(&dir);
    }
}
