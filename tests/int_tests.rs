use assert_cmd::cargo;

#[cfg(unix)]
mod common;
#[cfg(unix)]
use common::DefaultsFixture;

#[cfg(unix)]
const SINGLE_OUTPUT_DEFAULTS: &str = "amount: 100000000
input_currency:
  BitcoinUnit: SAT
output_currencies:
  - BitcoinUnit: BTC
";

#[cfg(unix)]
#[test]
fn test_no_arguments() {
    let defaults = DefaultsFixture::new(SINGLE_OUTPUT_DEFAULTS);
    defaults
        .command()
        .assert()
        .success()
        .stdout("1 BTC\n")
        .stderr("");
}

#[cfg(unix)]
#[test]
fn test_one_argument() {
    let defaults = DefaultsFixture::new(SINGLE_OUTPUT_DEFAULTS);
    defaults
        .command()
        .arg("1")
        .assert()
        .success()
        .stdout("0.00000001 BTC\n")
        .stderr("");
}

#[cfg(unix)]
#[test]
fn test_two_arguments() {
    let defaults = DefaultsFixture::new(SINGLE_OUTPUT_DEFAULTS);
    defaults
        .command()
        .args(["1", "BTC"])
        .assert()
        .success()
        .stdout("1 BTC\n")
        .stderr("");
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

#[cfg(unix)]
#[test]
fn test_format() {
    let defaults = DefaultsFixture::new(
        "amount: 100000000
input_currency:
  BitcoinUnit: SAT
output_currencies:
  - BitcoinUnit: BTC
  - BitcoinUnit: SAT
  - BitcoinUnit: MSAT
",
    );
    defaults
        .command()
        .arg("-i")
        .assert()
        .success()
        .stderr("")
        .stdout(concat!(
            "Input: 100,000,000 SAT\n",
            "\n",
            " unit | amount          \n",
            "------+-----------------\n",
            " BTC  | 1               \n",
            " SAT  | 100,000,000     \n",
            " MSAT | 100,000,000,000 \n",
        ));
}
