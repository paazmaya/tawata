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
