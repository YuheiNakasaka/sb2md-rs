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

#[cfg(test)]
mod tests {
    use super::*;
    use sb2md::request::{ScrapboxLine, ScrapboxPage};

    #[test]
    fn test_heading() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("[* Test]".to_string()),
            ScrapboxLine::new("[** Test]".to_string()),
            ScrapboxLine::new("[*** Test]".to_string()),
            ScrapboxLine::new("[**** Test]".to_string()),
            ScrapboxLine::new("[***** Test]".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "#### Test\n### Test\n## Test\n# Test\n# Test\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_link() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("[https://example.com/ Test]".to_string()),
            ScrapboxLine::new("[Test https://example.com/]".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "[Test](https://example.com/)\n[Test](https://example.com/)\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_strong() {
        let page = ScrapboxPage::new(vec![ScrapboxLine::new("This is a [* Test].".to_string())]);
        let md = ToMd::new(page).convert();
        let expected = "This is a **Test**.\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_img() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("[https://gyazo.com/example.png]".to_string()),
            ScrapboxLine::new("[https://scrapbox.io/files/example.png]".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected =
            "![](https://gyazo.com/example.png)\n![](https://scrapbox.io/files/example.png)\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_list() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new(" test".to_string()),
            ScrapboxLine::new("  test".to_string()),
            ScrapboxLine::new("   test".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "- test\n  - test\n    - test\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_codeblock() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("code:test.rs".to_string()),
            ScrapboxLine::new(" println!(\"{}\", 1);".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "```rs\n println!(\"{}\", 1);\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_table() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("table:test".to_string()),
            ScrapboxLine::new(" A	B".to_string()),
            ScrapboxLine::new(" C	D".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "| A | B |\n|:--|:--|\n| C | D |\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_sb_link() {
        let page = ScrapboxPage::new(vec![ScrapboxLine::new("[test]".to_string())]);
        let md = ToMd::new(page).convert();
        let expected = "test\n";
        assert_eq!(expected, &md[..]);
    }

    #[test]
    fn test_complex_text() {
        let page = ScrapboxPage::new(vec![
            ScrapboxLine::new("- this is a [* test]. In details, [https://example.com/ link] should be shown. [https://scrapbox.io/files/test.png]".to_string()),
        ]);
        let md = ToMd::new(page).convert();
        let expected = "- this is a **test**. In details, [link](https://example.com/) should be shown. ![](https://scrapbox.io/files/test.png)\n";
        assert_eq!(expected, &md[..]);
    }
}
