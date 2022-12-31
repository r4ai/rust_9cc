use assert_cmd::Command;
use rust_9cc::cli;
use std::{fs, fs::File, io::Write};

fn run_cmd(input: &str, ans: i32) {
    dbg!(input);
    let tmp_dir_path = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "tests/tmp");
    let tmp_assembly_path = format!("{}/{}", tmp_dir_path, "tmp.s");

    fs::create_dir_all(&tmp_dir_path).unwrap();

    let _output = cli(vec!["cargo-run".to_string(), input.to_string()]);

    let mut tmp_file = File::create(tmp_assembly_path).unwrap();
    tmp_file.write_all(_output.as_bytes()).unwrap();
    let compile_cmd = std::process::Command::new("cc")
        .args(["-o", "tmp", "tmp.s"])
        .current_dir(&tmp_dir_path)
        .status()
        .unwrap();
    assert!(compile_cmd.success());
    let run_cmd = std::process::Command::new("./tmp")
        .current_dir(&tmp_dir_path)
        .status()
        .unwrap();

    assert_eq!(run_cmd.code().unwrap(), ans);
}

#[test]
fn run_assembly() {
    run_cmd("3+7", 10);
    run_cmd("3-1", 2);
    run_cmd(" 3 + 9 - 9", 3);
    run_cmd("99 +0  - 83      + 3    ", 19);
}
