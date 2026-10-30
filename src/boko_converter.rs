//! Ebook converter for EPUB, MOBI, AZW3, AZW, and KFX formats powered by `boko`.

use anyhow::{Context, Result};
use boko::export::{Exporter, MarkdownExporter};
use boko::{Book, Format};
use log::{debug, info, warn};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Cursor;
use std::path::Path;

use crate::common::{
    is_image_filename, normalize_asset_path, prepare_output_dirs, read_input_file,
    rewrite_markdown_image_links_with_map, save_markdown_file, save_markdown_page,
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

/// Allocates a collision-free filename inside the `images` directory.
fn disambiguate_image_filename(base_name: &str, used_names: &mut HashSet<String>) -> String {
    if used_names.insert(base_name.to_string()) {
        return base_name.to_string();
    }

    let path = Path::new(base_name);
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("image");
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");

    let mut counter = 1usize;
    loop {
        let candidate = if ext.is_empty() {
            format!("{}_{}", stem, counter)
        } else {
            format!("{}_{}.{}", stem, counter, ext)
        };
        if used_names.insert(candidate.clone()) {
            return candidate;
        }
        counter += 1;
    }
}

/// Lazily creates `out_dir/images` when images are present, writes every image asset
/// of the book using collision-safe filenames, and returns an asset-path-to-filename map.
fn extract_images(book: &Book, out_dir: &Path) -> Result<HashMap<String, String>> {
    let mut assets: Vec<&str> = book.list_assets().iter().map(String::as_str).collect();
    assets.sort_unstable();

    let img_dir = out_dir.join("images");
    let mut img_dir_created = false;
    let mut used_names = HashSet::new();
    let mut asset_map = HashMap::new();

    for asset in assets {
        let normalized_asset = normalize_asset_path(asset);
        let base_name = match Path::new(&normalized_asset)
            .file_name()
            .and_then(|n| n.to_str())
        {
            Some(n) if is_image_filename(n) => n,
            _ => continue,
        };

        let bytes = match book.load_asset(asset) {
            Ok(bytes) => bytes,
            Err(e) => {
                warn!("Failed to load image asset {:?}: {}", asset, e);
                eprintln!("Warning: failed to load image asset {}: {}", asset, e);
                continue;
            }
        };

        if !img_dir_created {
            if !img_dir.exists() {
                debug!("Creating images directory: {:?}", img_dir);
                fs::create_dir_all(&img_dir)
                    .with_context(|| format!("Failed to create images directory: {:?}", img_dir))?;
            }
            img_dir_created = true;
        }

        let final_name = disambiguate_image_filename(base_name, &mut used_names);
        let dest = img_dir.join(&final_name);
        debug!("Extracting image {:?} -> {:?}", asset, dest);
        fs::write(&dest, bytes)
            .with_context(|| format!("Failed to write extracted image: {:?}", dest))?;

        asset_map.insert(normalized_asset.clone(), final_name.clone());
        if let Some((_, without_root)) = normalized_asset.split_once('/') {
            asset_map
                .entry(without_root.to_string())
                .or_insert_with(|| final_name.clone());
        }
        asset_map.entry(base_name.to_string()).or_insert(final_name);
    }

    Ok(asset_map)
}

/// Converts EPUB, MOBI, AZW3, AZW, and KFX files to markdown using `boko`.
/// With `single_file` the whole book is written to `book.md`, otherwise one file per chapter.
///
/// # Errors
///
/// Returns an error if the format is unsupported, the input file cannot be read or parsed,
/// Markdown export fails, or output files cannot be written.
pub fn process_boko(path: &Path, autocorrect: bool, oxfmt: bool, single_file: bool) -> Result<()> {
    let format =
        Format::from_path(path).with_context(|| format!("Unsupported ebook format: {:?}", path))?;
    let data = read_input_file(path)?;
    let book =
        Book::from_bytes(&data, format).with_context(|| format!("Failed to open {:?}", path))?;
    drop(data);
    let (out_dir, _) = prepare_output_dirs(path, false)?;

    let asset_map = extract_images(&book, &out_dir)?;
    debug!("Extracted {} image assets from {:?}", asset_map.len(), path);

    let mut buf = Cursor::new(Vec::new());
    MarkdownExporter::new()
        .export(&book, &mut buf)
        .context("Failed to export markdown")?;
    let markdown = rewrite_markdown_image_links_with_map(
        &String::from_utf8_lossy(&buf.into_inner()),
        "./images",
        &asset_map,
    );

    let chapters = if single_file {
        None
    } else {
        let split = split_chapters(&markdown, book.spine().len());
        if split.is_none() {
            warn!(
                "Could not reliably split chapters for {:?}; falling back to book.md",
                path
            );
            eprintln!("Could not reliably split chapters; writing a single file.");
        }
        split
    };

    match chapters {
        Some(chapters) => {
            info!("Writing {} chapter files to {:?}", chapters.len(), out_dir);
            for (i, chapter) in chapters.iter().enumerate() {
                save_markdown_page(&out_dir, "chapter", i + 1, chapter, autocorrect, oxfmt)?;
            }
        }
        None => {
            info!("Writing single book.md file to {:?}", out_dir);
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
<item id="missing" href="missing.png" media-type="image/png"/>
</manifest>
<spine><itemref idref="c1"/><itemref idref="c2"/></spine>
</package>"#;
        let c1 = r#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>Alpha</title></head><body><h1>Alpha</h1><p>Hello Alpha</p><p><img src="pic.png" alt="Pic"/></p></body></html>"#;
        let c2 = r#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>Beta</title></head><body><h1>Beta</h1><p>Hello Beta</p></body></html>"#;
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
    fn test_process_boko_epub_per_chapter() {
        let dir = temp_dir("epub_chapters");
        let path = dir.join("sample.epub");
        fs::write(&path, make_epub()).unwrap();

        let r = process_boko(&path, true, false, false);
        assert!(r.is_ok(), "{:?}", r);

        let out_dir = dir.join("sample");
        let ch1 = fs::read_to_string(out_dir.join("chapter_001.md")).unwrap();
        let ch2 = fs::read_to_string(out_dir.join("chapter_002.md")).unwrap();
        assert!(ch1.contains("Alpha"), "ch1: {}", ch1);
        assert!(ch1.contains("./images/pic.png"), "ch1: {}", ch1);
        assert!(ch2.contains("Beta"), "ch2: {}", ch2);
        assert!(!out_dir.join("book.md").exists());
        assert_eq!(
            fs::read(out_dir.join("images").join("pic.png")).unwrap(),
            b"\x89PNG fake"
        );
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_boko_epub_single_file() {
        let dir = temp_dir("epub_single");
        let path = dir.join("sample.epub");
        fs::write(&path, make_epub()).unwrap();

        assert!(process_boko(&path, false, false, true).is_ok());

        let out_dir = dir.join("sample");
        let book_md = fs::read_to_string(out_dir.join("book.md")).unwrap();
        assert!(book_md.contains("Alpha"));
        assert!(book_md.contains("Beta"));
        assert!(book_md.contains("./images/pic.png"));
        assert!(!out_dir.join("chapter_001.md").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_boko_epub_without_images_skips_images_dir() {
        let container = br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
<rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#;
        let opf = br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id">
<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
<dc:identifier id="id">test-id</dc:identifier><dc:title>NoImg</dc:title><dc:language>en</dc:language>
</metadata>
<manifest>
<item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/>
</manifest>
<spine><itemref idref="c1"/></spine>
</package>"#;
        let c1 = br#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>Only</title></head><body><h1>Only</h1><p>Text only</p></body></html>"#;
        let epub = make_zip(&[
            ("mimetype", b"application/epub+zip"),
            ("META-INF/container.xml", container),
            ("OEBPS/content.opf", opf),
            ("OEBPS/c1.xhtml", c1),
        ]);

        let dir = temp_dir("epub_no_images");
        let path = dir.join("noimg.epub");
        fs::write(&path, epub).unwrap();

        assert!(process_boko(&path, false, false, false).is_ok());
        let out_dir = dir.join("noimg");
        assert!(out_dir.join("chapter_001.md").exists());
        assert!(!out_dir.join("images").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_process_boko_epub_fallback_when_chapter_split_ambiguous() {
        let container = br#"<?xml version="1.0"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
<rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles>
</container>"#;
        let opf = br#"<?xml version="1.0"?>
<package xmlns="http://www.idpf.org/2007/opf" version="3.0" unique-identifier="id">
<metadata xmlns:dc="http://purl.org/dc/elements/1.1/">
<dc:identifier id="id">test-id</dc:identifier><dc:title>Fallback</dc:title><dc:language>en</dc:language>
</metadata>
<manifest>
<item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/>
</manifest>
<spine><itemref idref="c1"/></spine>
</package>"#;
        let c1 = br#"<?xml version="1.0"?><html xmlns="http://www.w3.org/1999/xhtml"><head><title>Ch1</title></head><body><p>Part 1</p><hr/><p>Part 2</p></body></html>"#;
        let epub = make_zip(&[
            ("mimetype", b"application/epub+zip"),
            ("META-INF/container.xml", container),
            ("OEBPS/content.opf", opf),
            ("OEBPS/c1.xhtml", c1),
        ]);

        let dir = temp_dir("epub_fallback");
        let path = dir.join("fallback.epub");
        fs::write(&path, epub).unwrap();

        assert!(process_boko(&path, false, false, false).is_ok());
        let out_dir = dir.join("fallback");
        assert!(out_dir.join("book.md").exists());
        assert!(!out_dir.join("chapter_001.md").exists());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_disambiguate_image_filename_collisions() {
        let mut used = HashSet::new();
        assert_eq!(disambiguate_image_filename("pic.png", &mut used), "pic.png");
        assert_eq!(
            disambiguate_image_filename("pic.png", &mut used),
            "pic_1.png"
        );
        assert_eq!(
            disambiguate_image_filename("pic.png", &mut used),
            "pic_2.png"
        );
        assert_eq!(disambiguate_image_filename("raw", &mut used), "raw");
        assert_eq!(disambiguate_image_filename("raw", &mut used), "raw_1");
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
