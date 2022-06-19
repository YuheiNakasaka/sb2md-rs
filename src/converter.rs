use lazy_static::lazy_static;
use regex::Regex;

use crate::sbtype::{ScrapboxLine, ScrapboxPage};

lazy_static! {
    static ref RGX_CODE_BLOCK: Regex = Regex::new(r"^code:.+").unwrap();
    static ref RGX_CODE_BLOCK_WITH_EXT: Regex = Regex::new(r"^code:[^.]*\.([^.]*)$").unwrap();
    static ref RGX_TABLE: Regex = Regex::new(r"^table:(.*)$").unwrap();
    static ref RGX_SPACED_LINE: Regex = Regex::new(r"^[\s|\t]+").unwrap();
    static ref RGX_HEADING: Regex = Regex::new(r"^\[(\*+)\s([^\]]+)\]$").unwrap();
    static ref RGX_STRONG: Regex = Regex::new(r"\[(\*+)\s([^\]]+)\]").unwrap();
    static ref RGX_LINK_PREFIX: Regex = Regex::new(r"\[(https?://[^\s]*)\s([^\]]*)]").unwrap();
    static ref RGX_LINK_SUFFIX: Regex = Regex::new(r"\[([^\]]*)\s(https?://[^\s\]]*)]").unwrap();
    static ref RGX_GYAZO_IMG: Regex = Regex::new(r"\[(https://gyazo.com/[^\s\]]*)\]").unwrap();
    static ref RGX_SB_IMG: Regex = Regex::new(r"\[(https://scrapbox.io/files/[^\s\]]*)\]").unwrap();
    static ref RGX_LIST: Regex = Regex::new(r"^([\s|\t]+)([^\s|\t]+)").unwrap();
    static ref RGX_SB_LINK_WITH_LINK: Regex = Regex::new(r"\[([^\]]+)\]([^\(])").unwrap();
    static ref RGX_SB_LINK_WITHOUT_LINK: Regex = Regex::new(r"\[([^\[]+)\]").unwrap();
}

pub enum TokenType {
    CodeBlock,
    Table,
    Other,
}

pub struct ToMd {
    page: ScrapboxPage,
    token_type: TokenType,
    output: String,
}

impl ToMd {
    pub fn new_by_text(text: String) -> Self {
        let lines = text
            .split("\n")
            .map(|line| ScrapboxLine::new(line.to_string()))
            .collect();
        let page = ScrapboxPage::new(lines);
        Self {
            page,
            token_type: TokenType::Other,
            output: String::new(),
        }
    }

    pub fn new_by_sb_page(page: ScrapboxPage) -> Self {
        Self {
            page,
            token_type: TokenType::Other,
            output: String::new(),
        }
    }

    pub fn convert(&mut self) -> String {
        let mut table_header = false;
        for line in &self.page.lines {
            match self.token_type {
                TokenType::CodeBlock => {
                    if !RGX_SPACED_LINE.is_match(&line.text[..]) {
                        self.output.push_str("```\n\n");
                        self.token_type = TokenType::Other;
                    } else {
                        self.output.push_str(&format!("{}\n", line.text));
                    }
                }
                TokenType::Table => {
                    if !RGX_SPACED_LINE.is_match(&line.text[..]) {
                        self.token_type = TokenType::Other;
                        self.output.push('\n');
                    } else {
                        let texts = line.text.trim().split('\t').collect::<Vec<&str>>();
                        let texts = texts.join(" | ");
                        let texts = format!("{}{}{}\n", "| ", texts, " |");
                        if table_header {
                            self.output.push_str(&texts);
                            self.output.push_str(&format!("{}\n", "|:--|:--|"));
                            table_header = false;
                        } else {
                            self.output.push_str(&texts);
                        }
                    }
                }
                TokenType::Other => {
                    if RGX_CODE_BLOCK.is_match(&line.text[..]) {
                        let captures = RGX_CODE_BLOCK_WITH_EXT.captures(&line.text);
                        if captures.is_some() {
                            let ext = captures.unwrap().get(1).unwrap().as_str();
                            self.output.push_str(&format!("```{}\n", ext));
                        } else {
                            self.output.push_str("```\n");
                        }
                        self.token_type = TokenType::CodeBlock;
                    } else if RGX_TABLE.is_match(&line.text[..]) {
                        self.token_type = TokenType::Table;
                        table_header = true;
                    } else if RGX_HEADING.is_match(&line.text[..]) {
                        let captures = RGX_HEADING.captures(&line.text).unwrap();
                        let heading_level = &captures[1];
                        let heading_level = if heading_level.len() >= 4 {
                            1
                        } else {
                            5 - heading_level.len()
                        };
                        let heading_level = "#".repeat(heading_level);
                        let heading_text = &captures[2];
                        self.output
                            .push_str(&format!("{} {}\n", heading_level, heading_text));
                    } else {
                        // check if it includes link
                        let has_link = RGX_LINK_PREFIX.is_match(&line.text[..])
                            || RGX_LINK_SUFFIX.is_match(&line.text[..]);
                        // gyazo image to md
                        let replaced_text = RGX_GYAZO_IMG
                            .replace_all(&line.text[..], "![]($1)")
                            .into_owned();
                        // scrapbox image to md
                        let replaced_text = RGX_SB_IMG
                            .replace_all(&replaced_text, "![]($1)")
                            .into_owned();
                        // link to md
                        let replaced_text = RGX_LINK_PREFIX
                            .replace_all(&replaced_text, "[$2]($1)")
                            .into_owned();
                        let replaced_text = RGX_LINK_SUFFIX
                            .replace_all(&replaced_text, "[$1]($2)")
                            .into_owned();
                        // strong to md
                        let replaced_text = RGX_STRONG
                            .replace_all(&replaced_text, "**$2**")
                            .into_owned();
                        // sblink to md
                        let replaced_text = if has_link {
                            RGX_SB_LINK_WITH_LINK
                                .replace_all(&replaced_text, "$1$2")
                                .into_owned()
                        } else {
                            RGX_SB_LINK_WITHOUT_LINK
                                .replace_all(&replaced_text, "$1")
                                .into_owned()
                        };
                        // list to md
                        let captures = RGX_LIST.captures(&replaced_text);
                        if captures.is_some() {
                            let matched = captures.unwrap();
                            let indent = "  ".repeat(&matched[1].len() - 1);
                            let replaced_text = replaced_text.trim();
                            self.output
                                .push_str(&format!("{}- {}\n", indent, replaced_text));
                        } else {
                            self.output.push_str(&format!("{}\n", replaced_text));
                        }
                    }
                }
            }
        }
        self.output.to_owned()
    }
}
