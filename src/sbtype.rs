use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScrapboxPage {
    pub lines: Vec<ScrapboxLine>,
}

impl ScrapboxPage {
    pub fn new(lines: Vec<ScrapboxLine>) -> Self {
        Self { lines }
    }
}

#[derive(Deserialize)]
pub struct ScrapboxLine {
    pub text: String,
}

impl ScrapboxLine {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}
