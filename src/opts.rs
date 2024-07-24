use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::Url;

#[derive(Debug, Parser)]
#[command(
    name = "httpie",
    version = "1.0",
    about = "Implement a simple httpie with rust"
)]
pub struct Opts {
    #[command(subcommand)]
    pub subcommand: SubCommand,
}

// subCommand support different Http method
#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(name = "get", about = "get method, get some data")]
    Get(Get),
    #[command(name = "post", about = "post method, post some data")]
    Post(Post),
}

#[derive(Debug, Parser)]
pub struct Get {
    #[arg(value_parser = parse_url)]
    pub url: String,
}

// why? I need to understand this
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Debug, Parser)]
pub struct Post {
    #[arg(value_parser = parse_url)]
    pub url: String,
    #[arg(value_parser = parse_kv_pair)]
    pub body: Vec<KvPair>,
}

// why? add PartialEq
#[derive(Debug, Clone, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}
