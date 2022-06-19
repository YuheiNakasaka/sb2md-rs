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

fn main() {
  let text = "- this is a [* test]. In details, [https://example.com/ link] should be shown. [https://scrapbox.io/files/test.png]";
  let md = ToMd::new_by_text(text.to_string()).convert();
  println!("{}", md);
}
```
