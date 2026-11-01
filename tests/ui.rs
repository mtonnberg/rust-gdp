//! Compile real consumers against the built crate, requiring the intended diagnostic.
use std::{fs, path::PathBuf, process::Command};
#[test]
fn proof_contracts_are_enforced_by_rustc() {
    let deps = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_owned();
    let rlib = fs::read_dir(&deps)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("librust_gdp-")
                && p.extension().is_some_and(|e| e == "rlib")
        })
        .max_by_key(|p| fs::metadata(p).unwrap().modified().unwrap())
        .unwrap();
    let out = std::env::temp_dir().join(format!("rust-gdp-ui-{}", std::process::id()));
    fs::create_dir_all(&out).unwrap();
    let ui = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/ui");
    let mut cases: Vec<_> = fs::read_dir(ui)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "rs"))
        .collect();
    cases.sort();
    for case in cases {
        let output = Command::new(std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into()))
            .arg("--edition=2021")
            .arg("--emit=metadata")
            .arg("--crate-name=ui_case")
            .arg("--extern")
            .arg(format!("rust_gdp={}", rlib.display()))
            .arg("-L")
            .arg(format!("dependency={}", deps.display()))
            .arg("--out-dir")
            .arg(&out)
            .arg(&case)
            .output()
            .unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        if case.file_stem().unwrap() == "positive" {
            assert!(output.status.success(), "{}: {stderr}", case.display());
        } else {
            let expected = fs::read_to_string(case.with_extension("stderr")).unwrap();
            assert!(
                !output.status.success() && stderr.contains(expected.trim()),
                "{}: expected rejection containing {:?}, got {stderr}",
                case.display(),
                expected.trim()
            );
        }
    }
    fs::remove_dir_all(out).unwrap();
}
