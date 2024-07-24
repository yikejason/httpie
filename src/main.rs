use anyhow::Result;
use clap::Parser;
use httpie::{get, post, Opts, SubCommand};
use reqwest::{header, Client};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Opts::parse();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = Client::builder().default_headers(headers).build()?;
    match opts.subcommand {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(())
}
