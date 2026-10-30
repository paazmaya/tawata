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
    let entries: Vec<_> = fs::read_dir(&out_dir).unwrap().collect();
    assert!(!entries.is_empty(), "no output files");
    // inspect output content
    let first = fs::read_dir(&out_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let content = fs::read_to_string(&first).unwrap();
    assert!(content.starts_with("# Page "));
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
    let _ = fs::remove_dir_all(&tmp);
}
