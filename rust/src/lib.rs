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
        // unterminated codespan
        &"aaa ` b c ` `ddd e",
        3,
        "aaa\n` b c `\n`ddd e",
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
