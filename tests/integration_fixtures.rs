use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_tawata"))
}

fn copy_fixture(name: &str) -> (PathBuf, PathBuf) {
    let tmp = std::env::temp_dir().join(format!("tawata_int_{}_{}", name, std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let src = PathBuf::from("fixtures").join(name);
    let dst = tmp.join(name);
    fs::copy(&src, &dst).unwrap();
    (tmp, dst)
}

#[test]
fn integration_djvu_fixture() {
    let (tmp, input) = copy_fixture("TolmanNon1.djvu");
    let output = Command::new(bin())
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = tmp.join("TolmanNon1");
    assert!(out_dir.exists(), "output dir missing");
    let mut entries: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    assert!(!entries.is_empty(), "no output files");

    let page_1 = out_dir.join("page_001.md");
    assert!(page_1.exists(), "page_001.md missing");
    let content = fs::read_to_string(&page_1).unwrap();
    assert!(content.starts_with("# Page 1\n"));
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_epub_fixture() {
    let (tmp, input) = copy_fixture("Iracema_by_José_Martiniano_de_Alencar.epub");
    let output = Command::new(bin())
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = tmp.join("Iracema_by_José_Martiniano_de_Alencar");
    assert!(out_dir.exists(), "output dir missing");

    let md_files: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    assert!(!md_files.is_empty(), "expected generated markdown files");

    let combined: String = md_files
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        combined.contains("Iracema") || combined.contains("Alencar"),
        "expected book content to mention Iracema or Alencar"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_azw3_fixture() {
    let (tmp, input) = copy_fixture("john-william-polidori_the-vampire.azw3");
    let output = Command::new(bin())
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = tmp.join("john-william-polidori_the-vampire");
    assert!(out_dir.exists(), "output dir missing");

    let md_files: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    assert!(!md_files.is_empty(), "expected generated markdown files");

    let combined: String = md_files
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        combined.contains("Vampyre")
            || combined.contains("Polidori")
            || combined.contains("Ruthven"),
        "expected book content from The Vampyre"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_mobi_fixture() {
    let (tmp, input) = copy_fixture("The_Strange_Case_of_Dr.Jekyll_and_Mr.Hyde.mobi");
    let output = Command::new(bin())
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = tmp.join("The_Strange_Case_of_Dr.Jekyll_and_Mr.Hyde");
    assert!(out_dir.exists(), "output dir missing");

    let md_files: Vec<_> = fs::read_dir(&out_dir)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "md"))
        .collect();
    assert!(!md_files.is_empty(), "expected generated markdown files");

    let combined: String = md_files
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        combined.contains("Jekyll") && combined.contains("Hyde"),
        "expected book content to mention Jekyll and Hyde"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_mobi_kf8_fixture() {
    let (tmp, input) = copy_fixture("The_Strange_Case_of_Dr.Jekyll_and_Mr.Hyde-kf8.mobi");
    let output = Command::new(bin())
        .arg("--single-file")
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out_dir = tmp.join("The_Strange_Case_of_Dr.Jekyll_and_Mr.Hyde-kf8");
    let book_md = out_dir.join("book.md");
    assert!(book_md.exists(), "book.md missing");

    let content = fs::read_to_string(&book_md).unwrap();
    assert!(
        content.contains("Jekyll") && content.contains("Hyde"),
        "expected book.md to mention Jekyll and Hyde"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_pdf_fixture_unsupported() {
    let (tmp, input) = copy_fixture("Council_Regulation_EU_No_216-2013.pdf");
    let output = Command::new(bin())
        .arg(&input)
        .output()
        .expect("failed to run tawata");
    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("Extension 'pdf' is not currently supported"),
        "stderr: {}",
        stderr
    );
    assert!(!tmp.join("Council_Regulation_EU_No_216-2013").exists());
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn integration_corrupted_ebook_exits_nonzero() {
    let tmp = std::env::temp_dir().join(format!("tawata_int_corrupt_{}", std::process::id()));
    let _ = fs::remove_dir_all(&tmp);
    fs::create_dir_all(&tmp).unwrap();
    let bad_epub = tmp.join("corrupt.epub");
    fs::write(&bad_epub, b"not a valid epub archive").unwrap();

    let output = Command::new(bin())
        .arg(&bad_epub)
        .output()
        .expect("failed to run tawata");
    assert!(
        !output.status.success(),
        "expected non-zero exit code for corrupt epub"
    );
    let _ = fs::remove_dir_all(&tmp);
}
