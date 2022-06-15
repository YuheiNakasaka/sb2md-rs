# sb2md

sb2md is a tool for converting Scrapbox to Markdown.

# INSTALL

## CLI

```
cargo install --git https://github.com/YuheiNakasaka/sb2md-rs
```

## LIB

```
[dependencies]
sb2md = { git = "https://github.com/YuheiNakasaka/sb2md-rs" }
```

# USAGE

## CLI

```
cargo run razokulover-tech-memo/scrapbox記法をmarkdownに変換するparserをrustで書く
```

## LIB

```
use sb2md::converter::ToMd;
use sb2md::request::{ScrapboxPage, ScrapboxLine};

fn main() {
  let page = ScrapboxPage::new(vec![
      ScrapboxLine::new("[* Test]".to_string()),
      ScrapboxLine::new(" test".to_string()),
      ScrapboxLine::new("  [https://example.com/ Example]".to_string()),
  ]);
  let md = ToMd::new(page).convert();
  println!("{}", md);
}
```
