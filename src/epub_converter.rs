use anyhow::{Context, Result};
use epub::doc::EpubDoc;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_epub(epub_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open EPUB
    let mut doc = EpubDoc::new(epub_path.to_str().unwrap()).context("Failed to open EPUB")?;

    // 2. Create output directory next to the EPUB
    let file_stem = epub_path.file_stem().unwrap().to_str().unwrap();
    let parent = epub_path.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = parent.join(file_stem);

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }
    let img_dir = out_dir.join("images");
    if !img_dir.exists() {
        fs::create_dir(&img_dir)?;
    }

    // 3. Extract Images
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

    for (id, img_name) in images {
        let img_path = img_dir.join(img_name);
        if let Some((content, _)) = doc.get_resource(&id) {
            fs::write(img_path, content)?;
        }
    }

    // 4. Extract Chapters to Markdown
    let img_regex = Regex::new(r"(!\[[^\]]*\])\([^)]*?([^/\\]+)\)").unwrap();
    let mut i = 0;
    loop {
        if let Some((chapter_content, _mime)) = doc.get_current_str() {
            let mut md_content = html2md::parse_html(&chapter_content);

            // Fix image paths to point to our local ./images/ folder
            md_content = img_regex
                .replace_all(&md_content, |caps: &regex::Captures| {
                    let alt_part = caps.get(1).unwrap().as_str();
                    let filename = caps.get(2).unwrap().as_str();
                    let is_image = filename.to_lowercase().ends_with(".jpg")
                        || filename.to_lowercase().ends_with(".png")
                        || filename.to_lowercase().ends_with(".webp");
                    if is_image {
                        format!("{}(./images/{})", alt_part, filename)
                    } else {
                        caps.get(0).unwrap().as_str().to_string()
                    }
                })
                .into_owned();

            // 5. Autocorrect logic
            if autocorrect {
                md_content = apply_autocorrect(&md_content);
            }

            // 6. Write to file
            let md_file = out_dir.join(format!("chapter_{:03}.md", i + 1));
            fs::write(&md_file, md_content)?;

            // 7. Format with oxfmt
            if oxfmt {
                format_with_oxfmt(&md_file)?;
            }

            i += 1;
        }

        if !doc.go_next() {
            break;
        }
    }

    println!("Successfully converted to {:?}", out_dir);
    Ok(())
}
fn apply_autocorrect(text: &str) -> String {
    // Use the `autocorrect` crate (standard in the epub2MD ecosystem)
    let text = autocorrect::format(text);

    // Regex to fix words broken by <span> tags in EPUBs (e.g., "w o r d" -> "word")
    let re = Regex::new(r"(?P<word>\b\w(?: \w)+\b)").unwrap();
    let cleaned = re.replace_all(&text, |caps: &regex::Captures| {
        caps.name("word").unwrap().as_str().replace(" ", "")
    });

    // Clean up multiple spaces
    let multi_space = Regex::new(r" {2,}").unwrap();
    multi_space.replace_all(&cleaned, " ").into_owned()
}

fn format_with_oxfmt(md_path: &Path) -> Result<()> {
    // Oxc/Oxfmt is primarily a CLI/Node tool. The most stable way to use it from Rust
    // is to call the CLI binary directly rather than using internal/alpha Rust APIs.
    let status = std::process::Command::new("oxfmt")
        .arg(md_path.to_str().unwrap())
        .status()
        .context("Failed to run oxfmt. Ensure it is installed globally (npm i -g oxfmt).")?;

    if !status.success() {
        eprintln!("Warning: oxfmt failed to format {:?}", md_path);
    }
    Ok(())
}
