use assert_cmd::Command;

#[test]
fn test_different_amount_of_arguments() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();

    // No argument
    cmd.assert().success();

    // 1 argument
    cmd.args(vec!["1"]).assert().success();

    // 2 arguments
    cmd.args(vec!["BTC"]).assert().success();

    // 3 arguments
    cmd.args(vec!["SAT"]).assert().stdout("100,000,000 SAT\n");
}

#[test]
fn test_clean_mode() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-c", "1", "BTC", "SAT"])
        .assert()
        .stdout("100000000\n");
}

#[test]
fn test_integer_mode() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-i", "1234567", "MSAT", "SAT"])
        .assert()
        .stdout("1,235 SAT\n");
}

#[test]
fn test_clean_and_integer_mode() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-ci", "1234567", "MSAT", "SAT"])
        .assert()
        .stdout("1235\n");
}

#[test]
#[ignore] // only run in CI, because local installations may have different currencies configured
fn test_format() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();

    let stdout = cmd.arg("-i").assert().get_output().stdout.clone();
    let stdout = String::from_utf8(stdout).unwrap();
    let stdout_lines: Vec<_> = stdout.split("\n").collect();

    assert_eq!(stdout_lines.get(0).unwrap(), &" unit | amount          "); // table header
    assert_eq!(stdout_lines.get(1).unwrap(), &"------+-----------------"); // header separator
    assert_eq!(stdout_lines.get(2).unwrap(), &" BTC  | 1               ");
    assert_eq!(stdout_lines.get(3).unwrap(), &" SAT  | 100,000,000     ");
    assert_eq!(stdout_lines.get(4).unwrap(), &" MSAT | 100,000,000,000 ");
    assert!(stdout_lines.get(5).unwrap().contains(" USD  | "));
    assert!(stdout_lines.get(6).unwrap().contains(" EUR  | "));
    assert!(stdout_lines.get(7).unwrap().contains(" GBP  | "));
    assert_eq!(stdout_lines.get(8).unwrap(), &""); // End with a newline to be POSIX compliant
}
