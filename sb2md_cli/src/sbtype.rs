use serde::Deserialize;

#[derive(Deserialize)]
pub struct ScrapboxPage {
    pub lines: Vec<ScrapboxLine>,
}

impl ScrapboxPage {
    pub fn new(lines: Vec<ScrapboxLine>) -> Self {
        Self { lines }
    }

    pub fn to_text(self) -> String {
        self.lines
            .iter()
            .map(|line| line.text.to_string())
            .collect::<Vec<String>>()
            .join("\n")
            .to_owned()
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
