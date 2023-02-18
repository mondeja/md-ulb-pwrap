mod pwrap;
mod codespan_parser;

use crate::pwrap::{MarkdownParagraphWrapper};

pub fn ulb_wrap_paragraph(
    text: &str,
    width: usize,
    first_line_width: usize,
) -> String {
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
        // square bracket don't break lines
        &"aa]\nbb\n[cc",
        1,
        "aa]\nbb\n[cc",
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
        "parámetro\nde ancho\nde",
    )]
    #[case(
        // Scriptio continua
        &concat!(
            "支持",
        ),
        10,
        "",
    )]
    fn ulb_wrap_paragraph_test(
        #[case] text: &str,
        #[case] width: usize,
        #[case] expected: String,
    ) {
        assert_eq!(
            ulb_wrap_paragraph(text, width, width),
            expected,
        );
    }
}
