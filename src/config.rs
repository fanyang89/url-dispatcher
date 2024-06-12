use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub browsers: HashMap<String, Browser>,
    pub general: General,
}

pub fn new(name: &str) -> Config {
    Config::new(name)
}

impl Config {
    pub(crate) fn new(name: &str) -> Config {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(name))
            .build()
            .unwrap();
        settings.try_deserialize::<Config>().unwrap()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Browser {
    pub path: String,
    pub args: Vec<String>,
}

impl Browser {
    pub fn args(&self, url: &String) -> Vec<String> {
        let mut args = self.args.clone();
        args.push(url.clone());
        args
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct General {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Rule {
    #[serde(rename = "re")]
    pub re_str: Option<String>,
    pub to: String,
}

impl Rule {
    pub fn re(&self) -> Option<Regex> {
        match &self.re_str {
            None => None,
            Some(s) => Some(Regex::new(s.as_str()).unwrap()),
        }
    }
}
