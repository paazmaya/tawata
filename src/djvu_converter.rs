use anyhow::{Context, Result};
use djvu_rs::Document;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn process_djvu(djvu_path: &Path, autocorrect: bool, oxfmt: bool) -> Result<()> {
    // 1. Open DJVU
    let doc = Document::open(djvu_path).context("Failed to open DJVU file")?;

    // 2. Create output directory next to the DJVU file
    let file_stem = djvu_path.file_stem().unwrap().to_str().unwrap();
    let parent = djvu_path.parent().unwrap_or_else(|| Path::new("."));
    let out_dir = parent.join(file_stem);

    if !out_dir.exists() {
        fs::create_dir(&out_dir)?;
    }

    let page_count = doc.page_count();
    println!("Extracting {} pages from DJVU...", page_count);

    for i in 0..page_count {
        let page = doc.page(i).context(format!("Failed to read page {}", i + 1))?;
        let text = if let Some(layer) = page.text_layer()? {
            layer.text
        } else {
            String::new()
        };

        let mut content = if text.trim().is_empty() {
            format!("# Page {}\n\n*(No text layer found on this page)*\n", i + 1)
        } else {
            format!("# Page {}\n\n{}\n", i + 1, text.trim())
        };

        // 3. Autocorrect logic
        if autocorrect {
            content = apply_autocorrect(&content);
        }

        // 4. Write to file
        let md_file = out_dir.join(format!("page_{:03}.md", i + 1));
        fs::write(&md_file, content)?;

        // 5. Format with oxfmt
        if oxfmt {
            format_with_oxfmt(&md_file)?;
        }
    }

    println!("Successfully converted DJVU to {:?}", out_dir);
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
