use anyhow::{Context, Result};
use boko::export::{Exporter, MarkdownExporter};
use boko::{Book, Format};
use std::fs;
use std::io::Cursor;
use std::path::Path;

use crate::common::{
    is_image_filename, prepare_output_dirs, read_input_file, rewrite_markdown_image_links,
    save_markdown_file, save_markdown_page,
};

/// Separator `MarkdownExporter` writes between spine entries.
const CHAPTER_SEPARATOR: &str = "\n---\n";

/// Splits boko's concatenated markdown into one string per spine entry.
/// Returns `None` when the split count does not match the spine length
/// (e.g. a chapter contains its own thematic break).
fn split_chapters(markdown: &str, expected: usize) -> Option<Vec<String>> {
    let parts: Vec<String> = markdown
        .split(CHAPTER_SEPARATOR)
        .map(|p| p.trim().to_string())
        .collect();
    (expected > 0 && parts.len() == expected).then_some(parts)
}

/// Writes every image asset of the book into `img_dir`, returning how many were written.
fn extract_images(book: &Book, img_dir: &Path) -> Result<usize> {
    let mut count = 0;
    for asset in book.list_assets() {
        let name = match Path::new(asset).file_name().and_then(|n| n.to_str()) {
            Some(n) if is_image_filename(n) => n,
            _ => continue,
        };
        if let Ok(bytes) = book.load_asset(asset) {
            fs::write(img_dir.join(name), bytes)?;
            count += 1;
        }
    }
    Ok(count)
}

/// Converts EPUB, MOBI, AZW3 and KFX files to markdown using `boko`.
/// With `single_file` the whole book is written to `book.md`, otherwise one file per chapter.
pub fn process_boko(path: &Path, autocorrect: bool, oxfmt: bool, single_file: bool) -> Result<()> {
    let format =
        Format::from_path(path).with_context(|| format!("Unsupported ebook format: {:?}", path))?;
    let data = read_input_file(path)?;
    let book =
        Book::from_bytes(&data, format).with_context(|| format!("Failed to open {:?}", path))?;
    drop(data);
    let (out_dir, img_dir) = prepare_output_dirs(path, true)?;

    if let Some(ref img_dir) = img_dir {
        extract_images(&book, img_dir)?;
    }

    let mut buf = Cursor::new(Vec::new());
    MarkdownExporter::new()
        .export(&book, &mut buf)
        .context("Failed to export markdown")?;
    let markdown =
        rewrite_markdown_image_links(&String::from_utf8_lossy(&buf.into_inner()), "./images");

    let chapters = if single_file {
        None
    } else {
        let split = split_chapters(&markdown, book.spine().len());
        if split.is_none() {
            eprintln!("Could not reliably split chapters; writing a single file.");
        }
        split
    };

    match chapters {
        Some(chapters) => {
            for (i, chapter) in chapters.iter().enumerate() {
                save_markdown_page(&out_dir, "chapter", i + 1, chapter, autocorrect, oxfmt)?;
            }
        }
        None => {
            save_markdown_file(&out_dir, "book.md", &markdown, autocorrect, oxfmt)?;
        }
    }

    println!("Successfully converted to {:?}", out_dir);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crc32(data: &[u8]) -> u32 {
        let mut crc = !0u32;
        for &b in data {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = if crc & 1 != 0 {
                    (crc >> 1) ^ 0xEDB8_8320
                } else {
                    crc >> 1
                };
            }
        }
        !crc
    }

    /// Builds an uncompressed ZIP archive in memory.
    fn make_zip(files: &[(&str, &[u8])]) -> Vec<u8> {
        let mut out = Vec::new();
        let mut central = Vec::new();
        for (name, data) in files {
            let offset = out.len() as u32;
            let crc = crc32(data);
            let mut common = Vec::new();
            common.extend_from_slice(&[20, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
            common.extend_from_slice(&crc.to_le_bytes());
            common.extend_from_slice(&(data.len() as u32).to_le_bytes());
            common.extend_from_slice(&(data.len() as u32).to_le_bytes());
            common.extend_from_slice(&(name.len() as u16).to_le_bytes());
            common.extend_from_slice(&0u16.to_le_bytes());
            out.extend_from_slice(&0x0403_4b50u32.to_le_bytes());
            out.extend_from_slice(&common);
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(data);

            central.extend_from_slice(&0x0201_4b50u32.to_le_bytes());
            central.extend_from_slice(&[20, 0]);
            central.extend_from_slice(&common);
            central.extend_from_slice(&[0; 10]);
            central.extend_from_slice(&offset.to_le_bytes());
            central.extend_from_slice(name.as_bytes());
        }
        let cd_offset = out.len() as u32;
        out.extend_from_slice(&central);
        out.extend_from_slice(&0x0605_4b50u32.to_le_bytes());
        out.extend_from_slice(&[0; 4]);
        out.extend_from_slice(&(files.len() as u16).to_le_bytes());
        out.extend_from_slice(&(files.len() as u16).to_le_bytes());
        out.extend_from_slice(&(central.len() as u32).to_le_bytes());
        out.extend_from_slice(&cd_offset.to_le_bytes());
        out.extend_from_slice(&0u16.to_le_bytes());
        out
    }

    fn make_epub() -> Vec<u8> {
        let container = br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
<rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#;
        let opf = br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id">
<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
<dc:identifier id="id">test-id</dc:identifier><dc:title>Test</dc:title><dc:language>en</dc:language>
</metadata>
<manifest>
<item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/>
<item id="c2" href="c2.xhtml" media-type="application/xhtml+xml"/>
<item id="img" href="pic.png" media-type="image/png"/>
</manifest>
<spine><itemref idref="c1"/><itemref idref="c2"/></spine>
</package>"#;
        let page = |t: &str| {
            format!(
                "<?xml version=\"1.0\"?><html xmlns=\"http://www.w3.org/1999/xhtml\"><head><title>{0}</title></head><body><h1>{0}</h1><p>Hello {0}</p></body></html>",
                t
            )
        };
        let c1 = page("Alpha");
        let c2 = page("Beta");
        make_zip(&[
            ("mimetype", b"application/epub+zip"),
            ("META-INF/container.xml", container),
            ("OEBPS/content.opf", opf),
            ("OEBPS/c1.xhtml", c1.as_bytes()),
            ("OEBPS/c2.xhtml", c2.as_bytes()),
            ("OEBPS/pic.png", b"\x89PNG fake"),
        ])
    }

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("tawata_boko_{}_{}", name, std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_process_boko_epub_per_chapter_and_single_file() {
        let dir = temp_dir("epub");
        let path = dir.join("sample.epub");
        fs::write(&path, make_epub()).unwrap();

        let r = process_boko(&path, true, false, false);
        assert!(r.is_ok(), "{:?}", r);
        assert!(process_boko(&path, false, false, true).is_ok());
        let produced = walkdir::WalkDir::new(&dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|x| x == "md"))
            .count();
        assert!(produced >= 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_boko_invalid_content() {
        let dir = temp_dir("invalid");
        let path = dir.join("broken.epub");
        fs::write(&path, b"not a zip").unwrap();
        assert!(process_boko(&path, false, false, false).is_err());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_boko_unsupported_extension() {
        assert!(process_boko(Path::new("file.xyz"), false, false, false).is_err());
    }

    #[test]
    fn test_process_boko_missing_file() {
        let result = process_boko(Path::new("does_not_exist_12345.epub"), false, false, false);
        assert!(result.is_err());
    }

    #[test]
    fn test_split_chapters_matches_count() {
        let md = "# A\n\ntext\n\n---\n\n# B\n\nmore\n";
        let parts = split_chapters(md, 2).unwrap();
        assert_eq!(parts, vec!["# A\n\ntext", "# B\n\nmore"]);
    }

    #[test]
    fn test_split_chapters_mismatch() {
        assert!(split_chapters("a\n---\nb\n---\nc", 2).is_none());
        assert!(split_chapters("a", 0).is_none());
    }
}
