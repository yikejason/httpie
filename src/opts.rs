use clap::Parser;

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
    pub url: String,
}

#[derive(Debug, Parser)]
pub struct Post {
    pub url: String,
    pub body: Vec<String>,
}
