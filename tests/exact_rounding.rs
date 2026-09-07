use assert_cmd::cargo;

#[test]
fn bitcoin_decimal_conversions_are_exact() {
    for (amount, from, to, expected) in [
        ("1500", "MSAT", "BTC", "0.000000015\n"),
        ("100500", "MSAT", "BITS", "1.005\n"),
        ("0.5", "MSAT", "BTC", "0.00000000001\n"),
        ("0.49999999999999999999999999999", "MSAT", "BTC", "0\n"),
        ("9007199254740993", "MSAT", "MSAT", "9007199254740993\n"),
        ("1.005e-3", "BITS", "SAT", "0.101\n"),
        ("1.005m", "BITS", "SAT", "0.101\n"),
        ("0.005", "SAT", "SAT", "0.005\n"),
    ] {
        cargo::cargo_bin_cmd!("bitcoinvert")
            .env("HTTPS_PROXY", "http://127.0.0.1:1")
            .args(["-c", amount, from, to])
            .assert()
            .success()
            .stdout(expected)
            .stderr("");
    }
}

#[test]
fn negative_midpoints_round_away_from_zero_without_negative_zero() {
    for (amount, expected) in [
        ("-0.5", "-1\n"),
        ("-0.49999999999999999999999999999", "0\n"),
    ] {
        cargo::cargo_bin_cmd!("bitcoinvert")
            .args(["-c", "--", amount, "MSAT", "MSAT"])
            .assert()
            .success()
            .stdout(expected)
            .stderr("");
    }
}

#[test]
fn out_of_range_conversion_returns_error_without_a_result() {
    let assertion = cargo::cargo_bin_cmd!("bitcoinvert")
        .args(["-c", "1e308", "BTC", "MSAT"])
        .assert()
        .code(exitcode::DATAERR)
        .stdout("");
    let stderr = String::from_utf8_lossy(&assertion.get_output().stderr);
    assert!(stderr.contains("outside the supported range"));
    assert!(!stderr.contains("panicked"));
}
