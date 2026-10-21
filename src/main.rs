use anyhow::Result;
use clap::Parser;

mod epub_converter;
mod mobi_converter;
mod resolver; // New module

#[derive(Parser, Debug)]
#[command(author, version, about = "Converts Ebooks (EPUB, MOBI) to Markdown.")]
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

        let result = match path.extension().and_then(|e| e.to_str()) {
            Some("epub") => epub_converter::process_epub(&path, cli.autocorrect, cli.oxfmt),

            Some("mobi") => mobi_converter::process_mobi(&path, cli.autocorrect, cli.oxfmt),

            // Handle other formats here in the future (e.g., AZW3 via kindling-mobi later)
            Some(ext) => {
                eprintln!(
                    "Skipping {:?}: Extension '{}' is not currently supported.",
                    path, ext
                );
                Ok(())
            }
            None => {
                eprintln!("Skipping {:?}: No file extension found.", path);
                Ok(())
            }
        };

        if let Err(e) = result {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    }

    Ok(())
}
