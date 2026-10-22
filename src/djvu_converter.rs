use anyhow::{Context, Result};
use djvu_rs::Document;
use std::path::Path;

use crate::common::{prepare_output_dirs, read_input_file, save_markdown_page};

/// Formats the extracted page text into markdown, providing a placeholder if no text was found.
pub fn format_page_markdown(page_num: usize, text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        format!(
            "# Page {}\n\n*(No text layer found on this page)*\n",
            page_num
        )
    } else {
        format!("# Page {}\n\n{}\n", page_num, trimmed)
    }
}

/// Converts a DJVU file to one markdown file per page using its text layer.
pub fn process_djvu(djvu_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open DJVU
    let data = read_input_file(djvu_path)?;
    let doc = Document::from_bytes(data).context("Failed to open DJVU file")?;

    // 2. Create output directory next to the DJVU file
    let (out_dir, _) = prepare_output_dirs(djvu_path, false)?;

    let page_count = doc.page_count();
    println!("Extracting {} pages from DJVU...", page_count);

    for i in 0..page_count {
        let page = doc
            .page(i)
            .context(format!("Failed to read page {}", i + 1))?;
        let text = if let Some(layer) = page.text_layer()? {
            layer.text
        } else {
            String::new()
        };

        let content = format_page_markdown(i + 1, &text);

        save_markdown_page(&out_dir, "page", i + 1, &content, autocorrect, oxfmt)?;
    }

    println!("Successfully converted DJVU to {:?}", out_dir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_djvu_minimal_page_without_text() {
        let mut data = b"AT&TFORM".to_vec();
        data.extend_from_slice(&22u32.to_be_bytes());
        data.extend_from_slice(b"DJVUINFO");
        data.extend_from_slice(&10u32.to_be_bytes());
        data.extend_from_slice(&[0, 10, 0, 10, 24, 0, 100, 0, 22, 0]);

        let dir = std::env::temp_dir().join(format!("tawata_djvu_min_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("min.djvu");
        std::fs::write(&path, data).unwrap();
        assert!(process_djvu(&path, false, false).is_ok());
        assert!(dir.join("min").exists() || std::fs::read_dir(&dir).unwrap().count() > 1);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_djvu_invalid_content() {
        let dir = std::env::temp_dir().join(format!("tawata_djvu_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("broken.djvu");
        std::fs::write(&path, b"not a djvu").unwrap();
        assert!(process_djvu(&path, false, false).is_err());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_djvu_missing_file() {
        let result = process_djvu(Path::new("does_not_exist_12345.djvu"), false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_format_page_markdown_empty_text() {
        let md = format_page_markdown(1, "");
        assert_eq!(md, "# Page 1\n\n*(No text layer found on this page)*\n");

        let md_whitespace = format_page_markdown(2, "   \n\t  ");
        assert_eq!(
            md_whitespace,
            "# Page 2\n\n*(No text layer found on this page)*\n"
        );
    }

    #[test]
    fn test_format_page_markdown_with_text() {
        let md = format_page_markdown(5, "This is page five content.");
        assert_eq!(md, "# Page 5\n\nThis is page five content.\n");
    }
}
