use assert_cmd::Command;
use predicates::str::contains;
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_cli_check_clean_file_exits_0() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "Clean line 1").unwrap();
    writeln!(temp, "Clean line 2").unwrap();
    temp.flush().unwrap();

    let mut cmd = Command::cargo_bin("machdown").unwrap();
    cmd.arg("check").arg(temp.path()).assert().success().code(0);
}

#[test]
fn test_cli_check_dirty_file_exits_1_with_diagnostics() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "First line  ").unwrap();
    writeln!(temp, "Second line").unwrap();
    temp.flush().unwrap();

    let mut cmd = Command::cargo_bin("machdown").unwrap();
    cmd.arg("check")
        .arg(temp.path())
        .assert()
        .failure()
        .code(1)
        .stdout(contains("MD009"))
        .stdout(contains("1:11"));
}

#[test]
fn test_cli_fix_dirty_file_removes_whitespace_and_exits_0() {
    let mut temp = NamedTempFile::new().unwrap();
    writeln!(temp, "First line  ").unwrap();
    writeln!(temp, "Second line  ").unwrap();
    temp.flush().unwrap();

    let path = temp.path().to_path_buf();

    let mut cmd_fix = Command::cargo_bin("machdown").unwrap();
    cmd_fix.arg("fix").arg(&path).assert().success().code(0);

    let content_after = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content_after, "First line\nSecond line\n");

    let mut cmd_check = Command::cargo_bin("machdown").unwrap();
    cmd_check.arg("check").arg(&path).assert().success().code(0);
}

#[test]
fn test_cli_missing_file_exits_2() {
    let mut cmd = Command::cargo_bin("machdown").unwrap();
    cmd.arg("check")
        .arg("non_existent_file_xyz_12345.md")
        .assert()
        .failure()
        .code(2);
}
