use assert_cmd::cargo;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use tempfile::TempDir;

pub fn bitcoinvert() -> assert_cmd::Command {
    let mut command = cargo::cargo_bin_cmd!("bitcoinvert");
    // Offline cases must fail promptly if they accidentally request fiat rates.
    command
        .env("HTTP_PROXY", "http://127.0.0.1:9")
        .env("HTTPS_PROXY", "http://127.0.0.1:9")
        .env("ALL_PROXY", "http://127.0.0.1:9")
        .env("NO_PROXY", "")
        .env("http_proxy", "http://127.0.0.1:9")
        .env("https_proxy", "http://127.0.0.1:9")
        .env("all_proxy", "http://127.0.0.1:9")
        .env("no_proxy", "")
        .env("RUST_LOG", "off");
    command
}

// dirs::home_dir honors HOME on Unix; Windows ignores environment overrides.
#[cfg(unix)]
pub struct DefaultsFixture {
    directory: TempDir,
}

#[cfg(unix)]
impl DefaultsFixture {
    pub fn new(contents: &str) -> Self {
        let directory = tempfile::tempdir().unwrap();
        let config_dir = directory.path().join(".config/bitcoinvert");
        fs::create_dir_all(&config_dir).unwrap();
        fs::write(config_dir.join("defaults.yaml"), contents).unwrap();
        Self { directory }
    }

    pub fn command(&self) -> assert_cmd::Command {
        let mut command = bitcoinvert();
        // Override only the subprocess environment so concurrent tests keep
        // separate defaults without touching the user's configuration.
        command.env("HOME", self.directory.path());
        command
    }
}
