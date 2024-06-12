use std::path::PathBuf;

use clap::Parser;

#[cfg(target_os = "windows")]
const DEFAULT_PATH: &str = "$HOME\\.config\\url-dispatcher\\config.toml";

#[cfg(any(target_os = "linux", target_os = "macos"))]
const DEFAULT_PATH: &str = "$HOME/.config/url-dispatcher/config.toml";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// URL to open
    pub url: String,

    /// Config path
    #[arg(short, long, default_value = DEFAULT_PATH)]
    pub config: Option<PathBuf>,
}

impl Cli {
    pub(crate) fn config_file(&self) -> Option<PathBuf> {
        match &self.config {
            None => None,
            Some(c) => {
                let home_dir = homedir::get_my_home()
                    .unwrap()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();
                Some(PathBuf::from(
                    c.to_str().unwrap().replace("$HOME", home_dir.as_str()),
                ))
            }
        }
    }
}
