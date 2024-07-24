use clap::Parser;
use httpie::{Opts, SubCommand};

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.subcommand {
        SubCommand::Get(get) => {
            println!("GET request to: {}", get.url);
        }
        SubCommand::Post(post) => {
            println!("POST request to: {}", post.url);
        }
    }
}
