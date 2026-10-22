use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use std::fs;
use std::path::Path;

use crate::common::{prepare_output_dirs, rewrite_markdown_image_links, save_markdown_page};

/// Converts HTML chapter content to markdown, rewriting embedded image links to local `./images/`.
pub fn convert_chapter_to_markdown(html_content: &str) -> String {
    let raw_md = html2md::parse_html(html_content);
    rewrite_markdown_image_links(&raw_md, "./images")
}

/// Extracts images embedded in the EPUB document into the destination `images` directory.
fn extract_images<R: std::io::Read + std::io::Seek>(
    doc: &mut EpubDoc<R>,
    img_dir: &Path,
) -> Result<usize> {
    let images: Vec<(String, String)> = doc
        .resources
        .iter()
        .filter(|(_, res)| res.mime.starts_with("image/"))
        .map(|(id, res)| {
            let img_name = Path::new(&res.path)
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or("unknown")
                .to_string();
            (id.clone(), img_name)
        })
        .collect();

    let count = images.len();
    for (id, img_name) in images {
        let img_path = img_dir.join(img_name);
        if let Some((content, _)) = doc.get_resource(&id) {
            fs::write(img_path, content)?;
        }
    }
    Ok(count)
}

pub fn process_epub(epub_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open EPUB
    let mut doc = EpubDoc::new(epub_path.to_str().unwrap()).context("Failed to open EPUB")?;

    // 2. Create output directory next to the EPUB
    let (out_dir, img_dir) = prepare_output_dirs(epub_path, true)?;

    // 3. Extract Images
    if let Some(ref img_dir) = img_dir {
        extract_images(&mut doc, img_dir)?;
    }

    // 4. Extract Chapters to Markdown
    let mut i = 0;
    loop {
        if let Some((chapter_content, _mime)) = doc.get_current_str() {
            let md_content = convert_chapter_to_markdown(&chapter_content);

            save_markdown_page(&out_dir, "chapter", i + 1, &md_content, autocorrect, oxfmt)?;
            i += 1;
        }

        if !doc.go_next() {
            break;
        }
    }

    println!("Successfully converted to {:?}", out_dir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_chapter_to_markdown_basic() {
        let html = "<h1>Title</h1><p>Paragraph with <b>bold</b> text.</p>";
        let md = convert_chapter_to_markdown(html);
        assert!(md.contains("Title"));
        assert!(md.contains("**bold**"));
    }

    #[test]
    fn test_convert_chapter_to_markdown_with_image() {
        let html = r#"<p>A photo: <img src="OEBPS/images/figure1.jpg" alt="Figure 1" /></p>"#;
        let md = convert_chapter_to_markdown(html);
        assert!(md.contains("![Figure 1](./images/figure1.jpg)"));
    }
}
