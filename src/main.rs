use clap::Parser;
use sb2md::converter::ToMd;
use sb2md::request::SbRequest;

#[derive(Parser, Debug)]
#[clap(
    author = "YuheiNakasaka",
    version = "0.0.1",
    about = "sb2md is a CLI for converting Scrapbox to Markdown",
    long_about = None
)]
struct Cli {
    path: String,
}

fn main() {
    let args = Cli::parse();
    let request = SbRequest::new(args.path);
    let resp = request.fetch().expect("failed to fetch");
    let md = ToMd::new(resp).convert();
    println!("{}", md);
}
