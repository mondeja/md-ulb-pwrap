//! [![Crate](https://img.shields.io/crates/v/md-ulb-pwrap?logo=rust)](https://crates.io/crates/md-ulb-pwrap) [![PyPI](https://img.shields.io/pypi/v/md-ulb-pwrap?logo=python&logoColor=white)](https://pypi.org/project/md-ulb-pwrap/)
//!
//! Markdown paragraph wrapper using [Unicode Line Breaking
//! Algorithm].
//!
//! Wrap a Markdown paragraph using a maximum desired width.
//! Only works for paragraphs that don't contain other
//! [container blocks]. Respects the prohibition against wrapping
//! text inside inline code blocks and links.
//!
//! [unicode line breaking algorithm]: https://unicode.org/reports/tr14/
//! [container blocks]: https://spec.commonmark.org/0.30/#container-blocks

mod parser;
pub mod pwrap;

use crate::pwrap::MarkdownParagraphWrapper;

pub fn ulb_wrap_paragraph(text: &str, width: usize, first_line_width: usize) -> String {
    MarkdownParagraphWrapper::new(text, first_line_width).wrap(width)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        &"aa bb cc",
        2,
        "aa\nbb\ncc",
    )]
    #[case(
        &"aa bb cc\n\n\n",
        2,
        "aa\nbb\ncc\n\n\n",
    )]
    #[case(
        &"\n\n\naa bb cc",
        2,
        "\n\n\naa\nbb\ncc",
    )]
    #[case(
        &"\n\n\naa bb cc\n\n\n",
        2,
        "\n\n\naa\nbb\ncc\n\n\n",
    )]
    #[case(
        &"aa bb cc\n",
        2,
        "aa\nbb\ncc\n",
    )]
    #[case(
        &"aaa bbb cc",
        3,
        "aaa\nbbb\ncc",
    )]
    #[case(
        &"aa bb cc",
        5,
        "aa bb\ncc",
    )]
    #[case(
        &"aa bb cc",
        50,
        "aa bb cc",
    )]
    #[case(
        &"a\n\n\né",
        80,
        "a\n\n\né",
    )]
    #[case(
        &"aaa `b` ccc",
        3,
        "aaa\n`b`\nccc",
    )]
    #[case(
        &"aaa ` ` ccc",
        3,
        "aaa\n` `\nccc",
    )]
    #[case(
        &"aaa ` ``  ``` a b c ` ccc",
        3,
        "aaa\n` ``  ``` a b c `\nccc",
    )]
    #[case(
        &"aaa ``` ``  ` a b c ``` ccc",
        3,
        "aaa\n``` ``  ` a b c ```\nccc",
    )]
    #[case(
        // unterminated codespan
        &"aaa ` b c ` `ddd e",
        3,
        "aaa\n` b c `\n`ddd e",
    )]
    #[case(
        // preserve linebreaks
        &"aaa ` b c ` `ddd\ne",
        3,
        "aaa\n` b c `\n`ddd\ne",
    )]
    #[case(
        // don't wrap at strong spans
        &"a **hola**",
        2,
        "a\n**hola**",
    )]
    #[case(
        &"a __hola__",
        2,
        "a\n__hola__",
    )]
    #[case(
        // don't wrap at italic spans
        &"a *hola*",
        2,
        "a\n*hola*",
    )]
    #[case(
        &"a _hola_",
        2,
        "a\n_hola_",
    )]
    #[case(
        // wrap inside italic and strong spans
        &"**hello hello**",
        4,
        "**hello\nhello**",
    )]
    #[case(
        &"*hello hello*",
        4,
        "*hello\nhello*",
    )]
    #[case(
        // LFCR newlines
        &"a\r\nb\r\nc\r\n",
        4,
        "a\r\nb\r\nc\r\n",
    )]
    #[case(
        // All LFCR newlines
        &"\r\n\r\n\r\n",
        4,
        "\r\n\r\n\r\n",
    )]
    #[case(
        // All newlines
        &"\n\n\n",
        4,
        "\n\n\n",
    )]
    #[case(
        // square bracket don't break lines
        &"aa]bb[cc",
        1,
        "aa]bb[cc",
    )]
    #[case(
        // text terminated on !
        &"aa bb cc!",
        2,
        "aa\nbb\ncc!",
    )]
    #[case(
        // text terminated on [
        &"aa bb cc[",
        2,
        "aa\nbb\ncc[",
    )]
    #[case(
        // text terminated on space
        &"aa bb cc d ",
        2,
        "aa\nbb\ncc\nd ",
    )]
    #[case(
        // text starting with en dash
        &"- aa bb",
        2,
        "-\naa\nbb",
    )]
    #[case(
        // inline image links
        // TODO: must wrap before link
        &"aa ![img alt](img-url)",
        1,
        "aa ![img\nalt](img-url)",
    )]
    #[case(
        &"aa![img alt](img-url 'Tit le')",
        1,
        "aa![img\nalt](img-url\n'Tit\nle')",
    )]
    #[case(
        // inline links
        &"aa [link text](link-url)",
        1,
        "aa\n[link\ntext](link-url)",
    )]
    #[case(
        &"aa[link text](link-url 'Tit le')",
        1,
        "aa[link\ntext](link-url\n'Tit\nle')",
    )]
    #[case(
        // image reference links
        // TODO: must wrap before link
        &"aa ![image alt][link-label]",
        1,
        "aa ![image\nalt][link-label]",
    )]
    #[case(
        &"aa![image alt][link-label]",
        1,
        "aa![image\nalt][link-label]",
    )]
    #[case(
        // reference links
        &"aa [link text][link-label]",
        1,
        "aa\n[link\ntext][link-label]",
    )]
    #[case(
        &"aa[link text][link-label]",
        1,
        "aa[link\ntext][link-label]",
    )]
    #[case(
    // TODO: breaking Commonmark spec at escaped space
    // inside link destination (see implementation
        // notes for details)
        &"[link text](link\\ destination 'link title')",
        4,
        "[link\ntext](link\\\ndestination\n'link\ntitle')",
    )]
    #[case(
        // Don't wrap on '/' character
        &"[foo bar](https://foo.bar/baz/qux/fox)",
        1,
        "[foo\nbar](https://foo.bar/baz/qux/fox)",
    )]
    #[case(
        // hard line breaks
        &"hard  \nline break",
        1,
        "hard  \nline\nbreak",
    )]
    #[case(
        &"hard\\\nline break",
        1,
        "hard\\\nline\nbreak",
    )]
    #[case(
        &"hard          \nline break",
        1,
        "hard          \nline\nbreak",
    )]
    #[case(
        &"hard\\          \nline break",
        1,
        "hard\\          \nline\nbreak",
    )]
    #[case(
        // space returns space
        &" ",
        1,
        " ",
    )]
    #[case(
        // empty string returns empty string
        &"",
        1,
        "",
    )]
    #[case(
        // newline returns newline
        &"\n",
        1,
        "\n",
    )]
    #[case(
        // zero width still works as 1
        &"\na b c d e\n",
        0,
        "\na\nb\nc\nd\ne\n",
    )]
    #[case(
        // maximum width
        &"a b c d e",
        usize::MAX,
        "a b c d e",
    )]
    #[case(
        // UTF-8 characters
        //
        // unicode-linebreak uses byte indexes of chars
        // to determine linebreak indexes, so if using
        // array character indexes the next text would
        // return something like 'parámetro d\ne ancho d\ne'
        &"parámetro de ancho de",
        10,
        "parámetro\nde ancho de",
    )]
    #[case(
        &"parámetro de ancho de caracteres de",
        10,
        "parámetro\nde ancho\nde\ncaracteres\nde",
    )]
    #[case(
        // Scriptio continua
        &concat!(
            "支持常见的温度传感器（例如，常见的热敏电阻、AD595、",
            "AD597、AD849x、PT100、PT1000、MAX6675、MAX31855、",
            "MAX31856、MAX31865、BME280、HTU21D和LM75）。",
            "还可以配置"
        ),
        10,
        concat!(
            "支持常见的温度传感器（例\n如，常见的热敏电阻、\n",
            "AD595、\nAD597、\nAD849x、\nPT100、\nPT1000、\n",
            "MAX6675、\nMAX31855、\nMAX31856、\nMAX31865、\n",
            "BME280、\nHTU21D和\nLM75）。还可以配\n置",
        ).to_string(),
    )]
    fn ulb_wrap_paragraph_test(#[case] text: &str, #[case] width: usize, #[case] expected: String) {
        assert_eq!(ulb_wrap_paragraph(text, width, width), expected);
    }
}
