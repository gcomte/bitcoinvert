use serde_json::{json, Value};

mod common;
use common::bitcoinvert;
#[cfg(unix)]
use common::DefaultsFixture;

#[test]
fn multiple_outputs_show_the_input_and_preserve_requested_order() {
    let output = bitcoinvert()
        .args(["100k", "SAT", "MSAT", "BTC", "SAT"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let output = String::from_utf8(output).unwrap();
    let lines: Vec<_> = output.lines().collect();
    assert_eq!(lines[0], "Input: 100,000 SAT");
    assert_eq!(lines[1], "");
    let rows: Vec<Vec<_>> = lines[4..]
        .iter()
        .map(|line| line.split('|').map(str::trim).collect())
        .collect();
    assert_eq!(
        rows,
        [
            ["MSAT", "100,000,000"],
            ["BTC", "0.001"],
            ["SAT", "100,000"],
        ]
    );
    assert!(output.ends_with('\n'));
}

#[test]
fn json_contains_decimal_strings_canonical_names_and_ordered_outputs() {
    let output = bitcoinvert()
        .args(["--json", "1k", "sats", "msat", "btc", "Sats", "btc"])
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        result,
        json!({
            "input": {"amount": "1000", "currency": "SAT"},
            "outputs": [
                {"amount": "1000000", "currency": "MSAT"},
                {"amount": "0.00001", "currency": "BTC"},
                {"amount": "1000", "currency": "SAT"},
                {"amount": "0.00001", "currency": "BTC"},
            ]
        })
    );
    assert!(output.ends_with(b"\n"));
}

#[test]
fn json_keeps_the_same_schema_for_one_output() {
    let output = bitcoinvert()
        .args(["1", "BTC", "SAT", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        result,
        json!({
            "input": {"amount": "1", "currency": "BTC"},
            "outputs": [{"amount": "100000000", "currency": "SAT"}]
        })
    );
}

#[cfg(unix)]
#[test]
fn json_uses_exact_configured_defaults_in_configured_output_order() {
    let defaults = DefaultsFixture::new(
        "amount: 9007199254740993.001
input_currency:
  BitcoinUnit: SAT
output_currencies:
  - BitcoinUnit: MSAT
  - BitcoinUnit: BTC
  - BitcoinUnit: SAT
",
    );
    let output = defaults
        .command()
        .arg("--json")
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(
        result,
        json!({
            "input": {"amount": "9007199254740993.001", "currency": "SAT"},
            "outputs": [
                {"amount": "9007199254740993001", "currency": "MSAT"},
                {"amount": "90071992.54740993001", "currency": "BTC"},
                {"amount": "9007199254740993.001", "currency": "SAT"},
            ]
        })
    );
}

#[test]
fn json_preserves_large_amounts_and_millisatoshi_precision() {
    for (amount, expected) in [
        (
            "9007199254740993",
            json!([
                {"amount": "9007199254740993", "currency": "MSAT"},
                {"amount": "90071.99254740993", "currency": "BTC"},
                {"amount": "9007199254740.993", "currency": "SAT"},
            ]),
        ),
        (
            "1",
            json!([
                {"amount": "1", "currency": "MSAT"},
                {"amount": "0.00000000001", "currency": "BTC"},
                {"amount": "0.001", "currency": "SAT"},
            ]),
        ),
    ] {
        let output = bitcoinvert()
            .args(["--json", amount, "MSAT", "MSAT", "BTC", "SAT"])
            .assert()
            .success()
            .stderr("")
            .get_output()
            .stdout
            .clone();
        let result: Value = serde_json::from_slice(&output).unwrap();
        assert_eq!(result["input"]["amount"], amount);
        assert_eq!(result["outputs"], expected);
    }
}

#[test]
fn later_conversion_overflow_leaves_no_partial_table_or_json() {
    for json in [false, true] {
        let mut command = bitcoinvert();
        if json {
            command.arg("--json");
        }
        command
            .args(["1e308", "BTC", "BTC", "MSAT"])
            .assert()
            .code(exitcode::DATAERR)
            .stdout("");
    }
}

#[test]
fn integer_rounding_is_applied_to_each_json_result() {
    let output = bitcoinvert()
        .args(["--json", "-i", "1234567", "MSAT", "SAT", "MSAT"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    let result: Value = serde_json::from_slice(&output).unwrap();
    assert_eq!(result["input"]["amount"], "1234567");
    assert_eq!(result["outputs"][0]["amount"], "1235");
    assert_eq!(result["outputs"][1]["amount"], "1234567");
}

#[test]
fn aliases_work_in_single_and_clean_output() {
    bitcoinvert()
        .args(["1", "BTC", "sats"])
        .assert()
        .success()
        .stdout("100,000,000 SAT\n")
        .stderr("");
    bitcoinvert()
        .args(["-c", "1k", "sats", "MSAT"])
        .assert()
        .success()
        .stdout("1000000\n")
        .stderr("");
}

#[cfg(unix)]
#[test]
fn clean_uses_a_single_configured_output() {
    let defaults = DefaultsFixture::new(
        "amount: 1234567
input_currency:
  BitcoinUnit: MSAT
output_currencies:
  - BitcoinUnit: SAT
",
    );
    defaults
        .command()
        .arg("--clean")
        .assert()
        .success()
        .stdout("1234.567\n")
        .stderr("");
}

#[cfg(unix)]
#[test]
fn clean_rejects_multiple_configured_outputs_before_fetching_rates() {
    let defaults = DefaultsFixture::new(
        "amount: 1
input_currency:
  Fiat: USD
output_currencies:
  - BitcoinUnit: BTC
  - Fiat: EUR
",
    );
    defaults
        .command()
        .arg("--clean")
        .assert()
        .code(exitcode::USAGE)
        .stdout("")
        .stderr("Clean output requires exactly one output currency; specify one, for example: bitcoinvert --clean 1 BTC SAT\n");
}

#[cfg(unix)]
#[test]
fn empty_configured_outputs_fail_in_every_format_before_fetching_rates() {
    let defaults = DefaultsFixture::new(
        "amount: 1
input_currency:
  Fiat: USD
output_currencies: []
",
    );
    for flag in [None, Some("--json"), Some("--clean")] {
        defaults
            .command()
            .args(flag)
            .assert()
            .code(exitcode::USAGE)
            .stdout("")
            .stderr("No output currencies are configured; specify at least one output currency, for example: bitcoinvert 1 BTC SAT\n");
    }
    defaults
        .command()
        .args(["1", "BTC", "SAT"])
        .assert()
        .success()
        .stdout("100,000,000 SAT\n")
        .stderr("");
}

#[test]
fn clean_rejects_multiple_outputs_before_fetching_input_or_output_rates() {
    for args in [
        ["-c", "1", "USD", "BTC", "SAT"],
        ["-c", "1", "BTC", "USD", "EUR"],
    ] {
        bitcoinvert()
            .args(args)
            .assert()
            .code(exitcode::USAGE)
            .stdout("")
            .stderr("Clean output requires exactly one output currency; specify one, for example: bitcoinvert --clean 1 BTC SAT\n");
    }
}

#[test]
fn clean_and_json_conflict_before_fetching_rates() {
    let result = bitcoinvert()
        .args(["-c", "--json", "1", "USD", "BTC"])
        .assert()
        .code(2)
        .stdout("")
        .get_output()
        .stderr
        .clone();
    let stderr = String::from_utf8(result).unwrap();
    assert!(stderr.contains("cannot be used with"));
    assert!(stderr.contains("--clean"));
    assert!(stderr.contains("--json"));
}

#[test]
fn invalid_later_output_fails_without_printing_or_fetching_rates() {
    bitcoinvert()
        .args(["1", "USD", "BTC", "invalid"])
        .assert()
        .code(exitcode::USAGE)
        .stdout("")
        .stderr("\"invalid\" is not a valid (output) currency!\n");
}

#[test]
fn help_documents_multi_output_and_scripting_examples() {
    let result = bitcoinvert()
        .arg("--help")
        .assert()
        .success()
        .stderr("")
        .get_output()
        .stdout
        .clone();
    let help = String::from_utf8(result).unwrap();
    assert!(help.contains("[OUTPUT_CURRENCY]..."));
    assert!(help.contains("bitcoinvert 100k SAT USD EUR GBP"));
    assert!(help.contains("bitcoinvert --clean 1 BTC SAT"));
    assert!(help.contains("bitcoinvert --json 1 BTC SAT MSAT"));
    assert!(help.contains("sats is an alias for SAT"));
}
