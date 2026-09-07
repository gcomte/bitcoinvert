use assert_cmd::cargo;

#[test]
fn invalid_amounts_return_usage_errors_without_output() {
    for amount in [
        "", " ", "μ", "㍃", "NaN", "nan", "inf", "infinity", "1e309", "1e308Y",
    ] {
        let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
        let result = cmd.args([amount, "BTC", "SAT"]).assert();
        result.code(exitcode::USAGE).stdout("");
    }
}

#[test]
fn multibyte_si_suffixes_are_parsed_as_complete_characters() {
    for amount in ["1μ", "1㍃"] {
        let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
        cmd.args(["-c", amount, "BTC", "SAT"])
            .assert()
            .success()
            .stdout("100\n")
            .stderr("");
    }
}

#[test]
fn valid_amount_formats_remain_supported() {
    for (amount, expected) in [
        ("0", "0\n"),
        ("1e2", "100\n"),
        ("1k", "1000\n"),
        ("1'000.25", "1000.25\n"),
    ] {
        let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
        cmd.args(["-c", amount, "SAT", "SAT"])
            .assert()
            .success()
            .stdout(expected)
            .stderr("");
    }
}

#[test]
fn invalid_explicit_output_is_rejected_before_fetching_rates() {
    for output in ["", "USDD", "☃"] {
        let mut cmd = cargo::cargo_bin_cmd!("bitcoinvert");
        cmd.env("HTTPS_PROXY", "http://127.0.0.1:1")
            .env("NO_PROXY", "")
            .args(["1", "USD", output])
            .assert()
            .code(exitcode::USAGE)
            .stdout("")
            .stderr(format!("\"{output}\" is not a valid (output) currency!\n"));
    }
}
