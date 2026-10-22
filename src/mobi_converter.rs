use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use crate::common::{prepare_output_dirs, rewrite_markdown_image_links, save_markdown_page};

/// Splits raw markdown converted from MOBI HTML into chapters based on markdown H1 headers.
pub fn split_mobi_chapters(markdown: &str) -> Vec<String> {
    let raw_parts = markdown.split("\n# ").collect::<Vec<&str>>();

    raw_parts
        .iter()
        .enumerate()
        .map(|(i, chapter)| {
            if i == 0 {
                chapter.to_string()
            } else {
                format!("# {}\n", chapter)
            }
        })
        .collect()
}

/// Converts continuous MOBI HTML into separate chapter markdown strings with rewritten image links.
pub fn convert_mobi_html_to_chapters(html_content: &str) -> Vec<String> {
    let md_content = html2md::parse_html(html_content);
    let md_fixed = rewrite_markdown_image_links(&md_content, "./images");
    split_mobi_chapters(&md_fixed)
}

/// Extracts images embedded in the MOBI document into the destination `images` directory.
fn extract_images(doc: &mobi::Mobi, img_dir: &Path) -> Result<usize> {
    let records = doc.image_records();
    for (i, image) in records.iter().enumerate() {
        let img_name = format!("image_{:03}.jpg", i);
        let img_path = img_dir.join(img_name);
        fs::write(img_path, image.content)?;
    }
    Ok(records.len())
}

pub fn process_mobi(mobi_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open and Parse the MOBI file
    let doc = mobi::Mobi::from_path(mobi_path).context("Failed to parse MOBI file")?;

    // 2. Create output directory next to the file
    let (out_dir, img_dir) = prepare_output_dirs(mobi_path, true)?;

    // 3. Extract Images
    if let Some(ref img_dir) = img_dir {
        extract_images(&doc, img_dir)?;
    }

    // 4. Extract Text/HTML and convert to chapters
    let raw_html = doc
        .content_as_string()
        .context("Failed to extract content from MOBI")?;
    let chapters = convert_mobi_html_to_chapters(&raw_html);

    for (i, chapter) in chapters.iter().enumerate() {
        save_markdown_page(&out_dir, "chapter", i + 1, chapter, autocorrect, oxfmt)?;
    }

    println!("Successfully converted MOBI to {:?}", out_dir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_mobi_chapters_single_without_heading() {
        let input = "Just introductory text without any headings.";
        let chapters = split_mobi_chapters(input);
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0], "Just introductory text without any headings.");
    }

    #[test]
    fn test_split_mobi_chapters_starts_with_heading() {
        let input = "# Chapter 1\nText 1\n# Chapter 2\nText 2";
        let chapters = split_mobi_chapters(input);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0], "# Chapter 1\nText 1");
        assert_eq!(chapters[1], "# Chapter 2\nText 2\n");
    }

    #[test]
    fn test_split_mobi_chapters_with_intro_and_headings() {
        let input = "Intro\n# Chapter 1\nFirst";
        let chapters = split_mobi_chapters(input);
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0], "Intro");
        assert_eq!(chapters[1], "# Chapter 1\nFirst\n");
    }

    #[test]
    fn test_convert_mobi_html_to_chapters() {
        let html = "<p>Intro</p><p>Chapter text</p>";
        let chapters = convert_mobi_html_to_chapters(html);
        assert_eq!(chapters.len(), 1);
        assert!(chapters[0].contains("Intro"));
        assert!(chapters[0].contains("Chapter text"));
    }

    #[test]
    fn test_convert_mobi_html_to_chapters_with_image() {
        let html = r#"<p>Some text</p><img src="cover.jpg" alt="Cover" />"#;
        let chapters = convert_mobi_html_to_chapters(html);
        assert_eq!(chapters.len(), 1);
        assert!(chapters[0].contains("![Cover](./images/cover.jpg)"));
    }
}
