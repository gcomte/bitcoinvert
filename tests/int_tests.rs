use assert_cmd::cargo;

#[test]
fn test_no_arguments() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.assert().success();
}

#[test]
fn test_one_argument() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(["1"]).assert().success();
}

#[test]
fn test_two_arguments() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(["1", "BTC"]).assert().success();
}

#[test]
fn test_three_arguments() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(["1", "BTC", "SAT"])
        .assert()
        .stdout("100,000,000 SAT\n");
}

#[test]
fn test_clean_mode() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-c", "1", "BTC", "SAT"])
        .assert()
        .stdout("100000000\n");
}

#[test]
fn test_integer_mode() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-i", "1234567", "MSAT", "SAT"])
        .assert()
        .stdout("1,235 SAT\n");
}

#[test]
fn test_clean_and_integer_mode() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-ci", "1234567", "MSAT", "SAT"])
        .assert()
        .stdout("1235\n");
}

#[test]
fn test_amount_input_validation() {
    // Throw error for arbitrary string inputs
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let arbitrary_string = "twentyone";
    cmd.args(vec![arbitrary_string, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{arbitrary_string}\" is not a valid amount!\n"));

    // Disallow using SI symbols as prefix
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let si_prefix = "M1";
    cmd.args(vec![si_prefix, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{si_prefix}\" is not a valid amount!\n"));

    // Allow using SI symbols as suffix
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let si_suffix = "1M";
    cmd.args(vec![&si_suffix, "SAT", "BTC"])
        .assert()
        .stdout("0.01 BTC\n".to_string());

    // Allow using floating point numbers
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let si_suffix_floating = "0.00012345";
    cmd.args(vec!["-ci", &si_suffix_floating, "BTC", "SAT"])
        .assert()
        .stdout("12345\n");

    // Allow using floating point numbers in combination with SI symbols as suffix
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let si_suffix_floating = "12.34k";
    cmd.args(vec![&si_suffix_floating, "SAT", "BTC"])
        .assert()
        .stdout("0.0001234 BTC\n");

    // Allow using floating point numbers with thousand separators
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let thousand_separated_float = "1'000 000,000.25";
    cmd.args(vec![&thousand_separated_float, "BITS", "BTC"])
        .assert()
        .stdout("1,000.00000025 BTC\n");

    // Print correct error message when only supplying thousand separators
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let thousand_separator = ", '";
    cmd.args(vec![&thousand_separator, "SAT", "BTC"])
        .assert()
        .stderr(format!("\"{thousand_separator}\" is not a valid amount!\n"));
}

#[test]
fn test_amount_output_rounding() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-c", "0.12345", "SAT", "MSAT"])
        .assert()
        .stdout("123\n");

    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-c", "0.6656", "SAT", "MSAT"])
        .assert()
        .stdout("666\n");

    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    cmd.args(vec!["-c", "90", "SAT", "BTC"])
        .assert()
        .stdout("0.0000009\n");

    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let stdout = cmd
        .args(vec!["-c", "0.123", "BTC", "USD"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let usd_value = String::from_utf8(stdout).unwrap();
    assert!(!usd_value.trim().is_empty(), "No conversion result");
    let fractional_places = usd_value
        .trim()
        .split_once('.')
        .map_or(0, |(_, fraction)| fraction.len());
    assert!(
        fractional_places <= 2,
        "Number has more than two decimal places"
    );

    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
    let stdout = cmd
        .args(vec!["-c", "21", "BTC", "JPY"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let jpy_value = String::from_utf8(stdout).unwrap();
    assert!(!jpy_value.trim().is_empty(), "No conversion result");
    assert!(!jpy_value.trim().contains('.'), "Number has decimal places");
}

#[test]
#[ignore] // only run in CI, because local installations may have different currencies configured
#[allow(clippy::get_first)]
fn test_format() {
    let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");

    let stdout = cmd.arg("-i").assert().get_output().stdout.clone();
    let stdout = String::from_utf8(stdout).unwrap();
    let stdout_lines: Vec<_> = stdout.split('\n').collect();

    assert_eq!(stdout_lines.get(0).unwrap(), &"Input: 100,000,000 SAT");
    assert_eq!(stdout_lines.get(1).unwrap(), &"");
    assert_eq!(stdout_lines.get(2).unwrap(), &" unit | amount          "); // table header
    assert_eq!(stdout_lines.get(3).unwrap(), &"------+-----------------"); // header separator
    assert_eq!(stdout_lines.get(4).unwrap(), &" BTC  | 1               ");
    assert_eq!(stdout_lines.get(5).unwrap(), &" SAT  | 100,000,000     ");
    assert_eq!(stdout_lines.get(6).unwrap(), &" MSAT | 100,000,000,000 ");
    assert!(stdout_lines.get(7).unwrap().contains(" USD  | "));
    assert!(stdout_lines.get(8).unwrap().contains(" EUR  | "));
    assert!(stdout_lines.get(9).unwrap().contains(" GBP  | "));
    assert_eq!(stdout_lines.get(10).unwrap(), &""); // End with a newline to be POSIX compliant
}
