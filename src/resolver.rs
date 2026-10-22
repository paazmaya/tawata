use anyhow::{Context, Result};
use glob::glob;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::common::is_supported_ebook_ext;

/// Checks whether a given path has an extension supported by the converters.
pub fn is_supported_ebook_path(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(is_supported_ebook_ext)
        .unwrap_or(false)
}

/// Recursively or non-recursively finds all supported ebook files within a directory.
pub fn collect_ebook_files_in_dir(dir: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    let depth = if recursive { usize::MAX } else { 1 };

    for entry in WalkDir::new(dir).max_depth(depth) {
        let entry = entry?;
        if entry.file_type().is_file() && is_supported_ebook_path(entry.path()) {
            paths.push(entry.into_path());
        }
    }
    Ok(paths)
}

/// Resolves an input string (glob pattern, directory, or direct file path) to a list of matching Paths.
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
            paths = collect_ebook_files_in_dir(path, recursive)?;
        } else if path.is_file() {
            paths.push(path.to_path_buf());
        } else {
            anyhow::bail!("Input path does not exist: {}", input);
        }
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_is_supported_ebook_path() {
        assert!(is_supported_ebook_path(Path::new("book.epub")));
        assert!(is_supported_ebook_path(Path::new("book.MOBI")));
        assert!(is_supported_ebook_path(Path::new("book.djvu")));
        assert!(is_supported_ebook_path(Path::new("book.DJV")));
        assert!(!is_supported_ebook_path(Path::new("book.pdf")));
        assert!(!is_supported_ebook_path(Path::new("book")));
    }

    #[test]
    fn test_collect_ebook_files_in_dir() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let test_dir = std::env::temp_dir().join(format!("resolver_test_{}", nanos));
        let sub_dir = test_dir.join("nested");
        fs::create_dir_all(&sub_dir).unwrap();

        File::create(test_dir.join("root.epub")).unwrap();
        File::create(test_dir.join("root.txt")).unwrap();
        File::create(sub_dir.join("nested.mobi")).unwrap();

        // Non-recursive should only find root.epub
        let non_rec = collect_ebook_files_in_dir(&test_dir, false).unwrap();
        assert_eq!(non_rec.len(), 1);
        assert_eq!(non_rec[0].file_name().unwrap(), "root.epub");

        // Recursive should find both root.epub and nested.mobi
        let rec = collect_ebook_files_in_dir(&test_dir, true).unwrap();
        assert_eq!(rec.len(), 2);

        // Cleanup
        let _ = fs::remove_dir_all(&test_dir);
    }

    #[test]
    fn test_resolve_inputs_nonexistent() {
        let result = resolve_inputs("non_existent_file_path_12345.epub", false);
        assert!(result.is_err());
    }
}
