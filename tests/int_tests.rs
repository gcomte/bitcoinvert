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
fn test_amount_input_validation() {
    // Throw error for arbitrary string inputs
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let arbitrary_string = "twentyone";
    cmd.args(vec![arbitrary_string, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{arbitrary_string}\" is not a valid amount!\n"));

    // Disallow using SI symbols as prefix
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let si_prefix = "M1";
    cmd.args(vec![si_prefix, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{si_prefix}\" is not a valid amount!\n"));

    // Allow using SI symbols as suffix
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let si_suffix = "1M";
    cmd.args(vec![&si_suffix, "SAT", "BTC"])
        .assert()
        .stdout(format!("0.01 BTC\n"));

    // Allow using floating point numbers
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let si_suffix_floating = "0.00012345";
    cmd.args(vec!["-ci", &si_suffix_floating, "BTC", "SAT"])
        .assert()
        .stdout("12345\n");

    // Allow using floating point numbers in combination with SI symbols as suffix
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let si_suffix_floating = "12.34k";
    cmd.args(vec![&si_suffix_floating, "SAT", "BTC"])
        .assert()
        .stdout("0.0001234 BTC\n");

    // Allow using floating point numbers with thousand separators
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let thousand_separated_float = "1'000 000,000.25";
    cmd.args(vec![&thousand_separated_float, "BITS", "BTC"])
        .assert()
        .stdout("1,000.00000025 BTC\n");

    // Print correct error message when only supplying thousand separators
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let thousand_separator = ", '";
    cmd.args(vec![&thousand_separator, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{thousand_separator}\" is not a valid amount!\n"));
}

#[test]
fn test_amount_output_rounding() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-c", "0.12345", "SAT", "MSAT"])
        .assert()
        .stdout("123\n");

    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-c", "0.6656", "SAT", "MSAT"])
        .assert()
        .stdout("666\n");

    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    cmd.args(vec!["-c", "90", "SAT", "BTC"])
        .assert()
        .stdout("0.0000009\n");

    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let stdout = cmd
        .args(vec!["-c", "0.123", "BTC", "USD"])
        .assert()
        .get_output()
        .stdout
        .clone();

    let usd_value = String::from_utf8(stdout)
        .unwrap()
        .trim()
        .parse::<f64>()
        .unwrap();

    let usd_value_scalar = (usd_value * 100.0).round();
    let reconstructed = usd_value_scalar / 100.0;
    assert!(
        (usd_value - reconstructed).abs() < f64::EPSILON,
        "Number has more than two decimal places"
    );

    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();
    let stdout = cmd
        .args(vec!["-c", "21", "BTC", "JPY"])
        .assert()
        .get_output()
        .stdout
        .clone();

    let jpy_value = String::from_utf8(stdout)
        .unwrap()
        .trim()
        .parse::<f64>()
        .unwrap();

    assert_eq!(jpy_value.round(), jpy_value, "Number has decimal places");
}

#[test]
#[ignore] // only run in CI, because local installations may have different currencies configured
#[allow(clippy::get_first)]
fn test_format() {
    let mut cmd = Command::cargo_bin("bitcoinvert").unwrap();

    let stdout = cmd.arg("-i").assert().get_output().stdout.clone();
    let stdout = String::from_utf8(stdout).unwrap();
    let stdout_lines: Vec<_> = stdout.split('\n').collect();

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
