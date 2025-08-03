use anyhow::Result;
use clap::Parser;
use reqwest::Url;


#[derive(Parser)]
#[clap(author="lan", version="0.1.0", about="A CLI for Custom tools", long_about = None)]
struct Args {
    #[clap(subcommand)]
    subcommand: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Httpie(HttpieArgs),
}

#[derive(Parser, Debug)]
struct HttpieArgs {
    #[clap(subcommand)]
    subcmd: HttpieSubCommand,
}

#[derive(Parser, Debug)]
enum HttpieSubCommand {
    Get(HttpieGetArgs),
    Post(HttpiePostArgs),
}

#[derive(Parser, Debug)]
struct HttpieGetArgs {
    #[clap(value_parser = parse_url)]
    #[clap(short, long)]
    url: String,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

#[derive(Parser, Debug)]
struct HttpiePostArgs {
    #[clap(value_parser = parse_url)]
    #[clap(short, long)]
    url: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.subcommand {
        Command::Httpie(httpie_args) => {
            match httpie_args.subcmd {
                HttpieSubCommand::Get(httpie_get_args) => {
                    println!("GET request to {}", httpie_get_args.url);
                }
                HttpieSubCommand::Post(httpie_post_args) => {
                    println!("POST request to {}", httpie_post_args.url);
                }
            }
        }
    }
}
