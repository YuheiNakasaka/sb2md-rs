use url::Url;

use crate::sbtype::ScrapboxPage;

pub struct SbRequest {
    pub url: String,
}

impl SbRequest {
    pub fn new(path: String) -> Self {
        let paths: Vec<String> = path.split('/').map(|p| p.to_string()).collect();
        let url = format!("https://scrapbox.io/api/pages/{}/{}", paths[0], paths[1]);
        let parsed_url = Url::parse(&url[..]).expect("url is invalid");
        Self {
            url: parsed_url.to_string(),
        }
    }

    pub fn fetch(&self) -> Result<ScrapboxPage, reqwest::Error> {
        let resp: ScrapboxPage = reqwest::blocking::get(&self.url)?.json()?;
        Result::Ok(resp)
    }
}
