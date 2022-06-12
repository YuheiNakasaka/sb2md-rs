use clap::Parser;
use serde::Deserialize;
use url::Url;

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

#[derive(Deserialize, Debug)]
struct ScrapboxPage {
    id: String,
    lines: Vec<ScrapboxLine>,
}

#[derive(Deserialize, Debug)]
struct ScrapboxLine {
    id: String,
    text: String,
}

struct SbRequest {
    url: String,
}

impl SbRequest {
    fn new(path: String) -> Self {
        let paths: Vec<String> = path.split("/").map(|p| p.to_string()).collect();
        let url = format!("https://scrapbox.io/api/pages/{}/{}", paths[0], paths[1]);
        let parsed_url = Url::parse(&url[..]).expect("url is invalid");
        Self {
            url: parsed_url.to_string(),
        }
    }

    fn fetch(&self) -> Result<ScrapboxPage, reqwest::Error> {
        let resp: ScrapboxPage = reqwest::blocking::get(&self.url)?.json()?;
        Result::Ok(resp)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let sbrequest = SbRequest::new(args.path);
    let resp = sbrequest.fetch().expect("failed to fetch");
    println!("{:?}", resp);
    Ok(())
}
