use anyhow::Result;
use clap::Parser;

mod converter;
mod resolver;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Converts EPUB to Markdown with image extraction."
)]
struct Cli {
    /// Input epub file, glob pattern (e.g. "*.epub"), or directory
    #[arg(required = true)]
    input: String,

    /// Recursively process directories
    #[arg(short, long)]
    recursive: bool,

    /// Run autocorrect on the markdown (fixes CJK/English spacing and broken EPUB words)
    #[arg(short, long)]
    autocorrect: bool,

    /// Format the resulting markdown files using oxfmt (requires oxfmt installed)
    #[arg(long, default_value_t = true)]
    oxfmt: bool,
}

fn main() -> Result<()> {
    env_logger::init();
    let cli = Cli::parse();

    // Resolve inputs (handles files, folders, globs, and recursion)
    let paths = resolver::resolve_inputs(&cli.input, cli.recursive)?;

    if paths.is_empty() {
        println!("No EPUB files found for input: {}", cli.input);
        return Ok(());
    }

    println!("Found {} EPUB files to process.", paths.len());

    for path in paths {
        println!("\nProcessing: {:?}", path);
        if let Err(e) = converter::process_epub(&path, cli.autocorrect, cli.oxfmt) {
            eprintln!("Error processing {:?}: {}", path, e);
        }
    }

    Ok(())
}
