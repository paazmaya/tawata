use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_mobi(mobi_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open and Parse the MOBI file
    let doc = mobi::Mobi::from_path(mobi_path).context("Failed to parse MOBI file")?;

    // 2. Create output directory next to the file
    let file_stem = mobi_path.file_stem().unwrap().to_str().unwrap();
    let parent = mobi_path.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = parent.join(file_stem);

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }
    let img_dir = out_dir.join("images");
    if !img_dir.exists() {
        fs::create_dir(&img_dir)?;
    }

    // 3. Extract Images
    for (i, image) in doc.image_records().iter().enumerate() {
        let img_name = format!("image_{:03}.jpg", i);
        let img_path = img_dir.join(img_name);

        fs::write(img_path, image.content)?;
    }

    // 4. Extract Text/HTML
    // MOBI files store text as a single continuous stream of HTML.
    // We must extract it and split it into manageable Markdown files.
    let raw_html = doc
        .content_as_string()
        .context("Failed to extract content from MOBI")?;
    let md_content = html2md::parse_html(&raw_html);

    // Fix image paths to point to our local ./images/ folder
    let img_regex = Regex::new(r"(!\[[^\]]*\])\([^)]*?([^/\\]+)\)").unwrap();
    let md_fixed = img_regex
        .replace_all(&md_content, |caps: &regex::Captures| {
            let alt_part = caps.get(1).unwrap().as_str();
            let filename = caps.get(2).unwrap().as_str();
            let is_image = filename.to_lowercase().ends_with(".jpg")
                || filename.to_lowercase().ends_with(".png");
            if is_image {
                format!("{}(./images/{})", alt_part, filename)
            } else {
                caps.get(0).unwrap().as_str().to_string()
            }
        })
        .into_owned();

    // 5. Split into chapters (Heuristic: Split on Markdown H1 or Page Breaks)
    let chapters = md_fixed.split("\n# ").collect::<Vec<&str>>();

    for (i, chapter) in chapters.iter().enumerate() {
        let mut content = if i == 0 && !chapter.starts_with("# ") {
            chapter.to_string()
        } else {
            format!("# {}\n", chapter)
        };

        // Apply Autocorrect
        if autocorrect {
            content = apply_autocorrect(&content);
        }

        // Write to file
        let md_file = out_dir.join(format!("chapter_{:03}.md", i + 1));
        fs::write(&md_file, content)?;

        // Format with oxfmt
        if oxfmt {
            format_with_oxfmt(&md_file)?;
        }
    }

    println!("Successfully converted MOBI to {:?}", out_dir);
    Ok(())
}



fn apply_autocorrect(text: &str) -> String {
    let text = autocorrect::format(text);
    let re = Regex::new(r"(?P<word>\b\w(?: \w)+\b)").unwrap();
    let cleaned = re.replace_all(&text, |caps: &regex::Captures| {
        caps.name("word").unwrap().as_str().replace(" ", "")
    });
    let multi_space = Regex::new(r" {2,}").unwrap();
    multi_space.replace_all(&cleaned, " ").into_owned()
}

fn format_with_oxfmt(md_path: &Path) -> Result<()> {
    let status = std::process::Command::new("oxfmt")
        .arg(md_path.to_str().unwrap())
        .status()
        .context("Failed to run oxfmt. Ensure it is installed globally (npm i -g oxfmt).")?;

    if !status.success() {
        eprintln!("Warning: oxfmt failed to format {:?}", md_path);
    }
    Ok(())
}
