mod cli;
mod config;

use crate::config::Browser;
use clap::Parser;
use cli::Cli;
use std::ffi::c_char;
use std::ffi::c_int;
use std::ffi::c_void;

link_args::windows::raw!(unsafe "/SUBSYSTEM:WINDOWS");

fn launch(browser: &Browser, url: &String) {
    let _ = std::process::Command::new(&browser.path)
        .args(&browser.args(&url))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "system" fn WinMain(
    _hInstance: *const c_void,
    _hPrevInstance: *const c_void,
    _lpCmdLine: *const c_char,
    _nCmdShow: c_int,
) -> c_int {
    main();
    0
}

fn main() {
    let args = Cli::parse();
    let config_name = &args.config_file().unwrap();
    let c = config::new(config_name.to_str().unwrap());

    for rule in c.general.rules.iter() {
        let to = &rule.to;
        let browser = c.browsers.get(to).unwrap();
        match rule.re() {
            None => {
                launch(browser, &args.url);
                break;
            }
            Some(re) => {
                if re.is_match(&args.url) {
                    launch(browser, &args.url);
                    break;
                }
            }
        }
    }
}
