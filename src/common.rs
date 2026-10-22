use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

static SPACED_WORDS_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?P<word>\b\w(?: \w)+\b)").unwrap());

static MULTI_SPACE_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r" {2,}").unwrap());

static MARKDOWN_IMAGE_RE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(!\[[^\]]*\])\([^)]*?([^/\\]+)\)").unwrap());

/// Checks whether a given file extension corresponds to a supported ebook format.
pub fn is_supported_ebook_ext(ext: &str) -> bool {
    matches!(
        ext.to_ascii_lowercase().as_str(),
        "epub" | "mobi" | "djvu" | "djv"
    )
}

/// Checks whether a given filename has a recognized image extension.
pub fn is_image_filename(filename: &str) -> bool {
    let lower = filename.to_ascii_lowercase();
    lower.ends_with(".jpg")
        || lower.ends_with(".jpeg")
        || lower.ends_with(".png")
        || lower.ends_with(".webp")
        || lower.ends_with(".gif")
        || lower.ends_with(".svg")
}

/// Rewrites markdown image references (e.g. `![alt](path/to/img.png)`)
/// to point to the local images directory prefix (e.g. `./images/img.png`).
pub fn rewrite_markdown_image_links(markdown: &str, img_dir_prefix: &str) -> String {
    let prefix = img_dir_prefix.trim_end_matches('/');
    MARKDOWN_IMAGE_RE
        .replace_all(markdown, |caps: &regex::Captures| {
            let alt_part = caps.get(1).unwrap().as_str();
            let filename = caps.get(2).unwrap().as_str();
            if is_image_filename(filename) {
                format!("{}({}/{})", alt_part, prefix, filename)
            } else {
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .into_owned()
}

/// Reconstructs words broken apart by spaces (e.g. "w o r d" -> "word").
pub fn clean_spaced_words(text: &str) -> String {
    SPACED_WORDS_RE
        .replace_all(text, |caps: &regex::Captures| {
            caps.name("word").unwrap().as_str().replace(' ', "")
        })
        .into_owned()
}

/// Collapses sequences of two or more spaces into a single space.
pub fn collapse_whitespace(text: &str) -> String {
    MULTI_SPACE_RE.replace_all(text, " ").into_owned()
}

/// Applies standard autocorrect formatting along with spaced-word and whitespace cleanups.
pub fn apply_autocorrect(text: &str) -> String {
    let formatted = autocorrect::format(text);
    let cleaned_words = clean_spaced_words(&formatted);
    collapse_whitespace(&cleaned_words)
}

/// Formats a page or chapter filename, such as "chapter_001.md" or "page_042.md".
pub fn format_page_filename(prefix: &str, index: usize) -> String {
    format!("{}_{:03}.md", prefix, index)
}

/// Determines the output directory path alongside the source file.
pub fn determine_output_dir(input_path: &Path) -> PathBuf {
    let file_stem = input_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let parent = input_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join(file_stem)
}

/// Creates the output directory and optionally an "images" directory within it.
pub fn prepare_output_dirs(
    input_path: &Path,
    need_images: bool,
) -> Result<(PathBuf, Option<PathBuf>)> {
    let out_dir = determine_output_dir(input_path);
    if !out_dir.exists() {
        fs::create_dir_all(&out_dir)
            .with_context(|| format!("Failed to create output directory: {:?}", out_dir))?;
    }

    let img_dir = if need_images {
        let dir = out_dir.join("images");
        if !dir.exists() {
            fs::create_dir_all(&dir)
                .with_context(|| format!("Failed to create images directory: {:?}", dir))?;
        }
        Some(dir)
    } else {
        None
    };

    Ok((out_dir, img_dir))
}

/// Prepares markdown text by optionally applying autocorrect.
pub fn prepare_markdown_content(content: &str, autocorrect: bool) -> String {
    if autocorrect {
        apply_autocorrect(content)
    } else {
        content.to_string()
    }
}

/// Formats a markdown file using external `oxfmt` CLI.
pub fn format_with_oxfmt(md_path: &Path) -> Result<()> {
    let path_str = md_path
        .to_str()
        .with_context(|| format!("Invalid UTF-8 in path: {:?}", md_path))?;

    let status = std::process::Command::new("oxfmt")
        .arg(path_str)
        .status()
        .context("Failed to run oxfmt. Ensure it is installed globally (npm i -g oxfmt).")?;

    if !status.success() {
        eprintln!("Warning: oxfmt failed to format {:?}", md_path);
    }
    Ok(())
}

/// Writes markdown content to `{out_dir}/{prefix}_{index:03}.md`, applying autocorrect
/// and oxfmt formatting if requested.
pub fn save_markdown_page(
    out_dir: &Path,
    prefix: &str,
    index: usize,
    content: &str,
    autocorrect: bool,
    oxfmt: bool,
) -> Result<PathBuf> {
    let prepared = prepare_markdown_content(content, autocorrect);
    let filename = format_page_filename(prefix, index);
    let md_file = out_dir.join(filename);

    fs::write(&md_file, prepared)
        .with_context(|| format!("Failed to write markdown file: {:?}", md_file))?;

    if oxfmt {
        format_with_oxfmt(&md_file)?;
    }

    Ok(md_file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_is_supported_ebook_ext() {
        assert!(is_supported_ebook_ext("epub"));
        assert!(is_supported_ebook_ext("EPUB"));
        assert!(is_supported_ebook_ext("mobi"));
        assert!(is_supported_ebook_ext("MOBI"));
        assert!(is_supported_ebook_ext("djvu"));
        assert!(is_supported_ebook_ext("DJV"));
        assert!(!is_supported_ebook_ext("pdf"));
        assert!(!is_supported_ebook_ext("txt"));
        assert!(!is_supported_ebook_ext(""));
    }

    #[test]
    fn test_is_image_filename() {
        assert!(is_image_filename("photo.jpg"));
        assert!(is_image_filename("photo.JPEG"));
        assert!(is_image_filename("cover.png"));
        assert!(is_image_filename("art.webp"));
        assert!(is_image_filename("anim.gif"));
        assert!(is_image_filename("vector.svg"));
        assert!(!is_image_filename("document.pdf"));
        assert!(!is_image_filename("chapter.html"));
        assert!(!is_image_filename("notes.txt"));
    }

    #[test]
    fn test_rewrite_markdown_image_links() {
        let input = "Here is an image: ![Cover](OEBPS/images/cover.jpg)\nAnd another: ![Diagram](assets/diag.png)\nAnd non-image: [Doc](files/doc.pdf)";
        let rewritten = rewrite_markdown_image_links(input, "./images");
        assert_eq!(
            rewritten,
            "Here is an image: ![Cover](./images/cover.jpg)\nAnd another: ![Diagram](./images/diag.png)\nAnd non-image: [Doc](files/doc.pdf)"
        );

        // Windows style backslashes
        let win_input = "![Alt text](folder\\sub\\photo.webp)";
        let win_rewritten = rewrite_markdown_image_links(win_input, "./images");
        assert_eq!(win_rewritten, "![Alt text](./images/photo.webp)");
    }

    #[test]
    fn test_rewrite_markdown_image_links_leaves_non_images() {
        let input = "![Not an image](assets/data.json)";
        assert_eq!(rewrite_markdown_image_links(input, "./images"), input);
    }

    #[test]
    fn test_clean_spaced_words() {
        assert_eq!(clean_spaced_words("w o r d"), "word");
        assert_eq!(
            clean_spaced_words("Spaced w o r d here"),
            "Spaced word here"
        );
        assert_eq!(
            clean_spaced_words("Normal text stays normal"),
            "Normal text stays normal"
        );
    }

    #[test]
    fn test_collapse_whitespace() {
        assert_eq!(collapse_whitespace("Hello    world!"), "Hello world!");
        assert_eq!(
            collapse_whitespace("No  extra   spaces    here"),
            "No extra spaces here"
        );
        assert_eq!(collapse_whitespace("Single space"), "Single space");
    }

    #[test]
    fn test_format_page_filename() {
        assert_eq!(format_page_filename("chapter", 1), "chapter_001.md");
        assert_eq!(format_page_filename("chapter", 42), "chapter_042.md");
        assert_eq!(format_page_filename("page", 123), "page_123.md");
    }

    #[test]
    fn test_determine_output_dir() {
        assert_eq!(
            determine_output_dir(Path::new("books/novel.epub")),
            Path::new("books/novel")
        );
        assert_eq!(
            determine_output_dir(Path::new("sample.mobi")),
            Path::new("sample")
        );
    }

    #[test]
    fn test_prepare_markdown_content() {
        let raw = "Hello    world!";
        let without_ac = prepare_markdown_content(raw, false);
        assert_eq!(without_ac, raw);

        let with_ac = prepare_markdown_content("Hello    w o r l d!", true);
        assert!(with_ac.contains("world"));
        assert!(!with_ac.contains("    "));
    }

    #[test]
    fn test_prepare_output_dirs_and_save_page() {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("ebook2md_test_{}", nanos));
        let fake_input = temp_dir.join("test_book.epub");

        let (out_dir, img_dir) = prepare_output_dirs(&fake_input, true).unwrap();
        assert!(out_dir.exists());
        assert!(img_dir.unwrap().exists());

        let written_file = save_markdown_page(
            &out_dir,
            "chapter",
            1,
            "# Chapter 1\n\nContent here.",
            false,
            false,
        )
        .unwrap();

        assert!(written_file.exists());
        let read_back = fs::read_to_string(&written_file).unwrap();
        assert_eq!(read_back, "# Chapter 1\n\nContent here.");

        // Cleanup test directory
        let _ = fs::remove_dir_all(&temp_dir);
    }
}
