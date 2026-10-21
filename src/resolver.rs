use anyhow::{Context, Result};
use glob::glob;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn resolve_inputs(input: &str, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();

    if input.contains('*') {
        // Handle Glob patterns
        for entry in glob(input).context("Failed to read glob pattern")? {
            match entry {
                Ok(path) => paths.push(path),
                Err(e) => eprintln!("Glob error: {:?}", e),
            }
        }
    } else {
        let path = Path::new(input);
        if path.is_dir() {
            let depth = if recursive { std::usize::MAX } else { 1 };
            for entry in WalkDir::new(path).max_depth(depth) {
                let entry = entry?;
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if ext.to_string_lossy().to_lowercase() == "epub" {
                            paths.push(entry.into_path());
                        }
                    }
                }
            }
        } else if path.is_file() {
            paths.push(path.to_path_buf());
        } else {
            anyhow::bail!("Input path does not exist: {}", input);
        }
    }

    Ok(paths)
}
