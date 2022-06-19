# sb2md

sb2md is a tool for converting Scrapbox to Markdown.

# INSTALL

## CLI

```
cargo install --git https://github.com/YuheiNakasaka/sb2md-rs --bin sb2md_cli
```

## LIB

```
[dependencies]
sb2md_converter = { git = "https://github.com/YuheiNakasaka/sb2md-rs" }
```

# USAGE

## CLI

```
sb2md_cli razokulover-tech-memo/scrapbox記法をmarkdownに変換するparserをrustで書く
```

## LIB

```
use sb2md_converter::ToMd;

fn main() {
  let text = "- this is a [* test]. In details, [https://example.com/ link] should be shown. [https://scrapbox.io/files/test.png]";
  let md = ToMd::new(text.to_string()).convert();
  println!("{}", md);
}
```
