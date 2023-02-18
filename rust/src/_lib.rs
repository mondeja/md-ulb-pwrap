/**
 * Notes for reimplementation:
 *
 * Just use byte indexes instead of char indexes.
 * The width to compare against must be updated for
 * each line in the state parser. If a byte index is
 * greater by one than the previous byte index, then
 * the width to compare against must be updated by
 * byte_index - 1.
 *
 * Use an iterator with byte indexes.
 **/

use unicode_linebreak::{
    linebreaks,
    BreakOpportunity,
    BreakOpportunity::{Mandatory},
};

// Codespan parser states
static INSIDE_TEXT: u8 = 0b1;
static ENTERING_CODESPAN: u8 = 0b10;
static INSIDE_CODESPAN: u8 = 0b100;
static EXITING_CODESPAN: u8 = 0b1000;

pub struct MarkdownParagraphWrapper<'a> {
    text_iterator: std::iter::Enumerate<std::str::Chars<'a>>,
    state_parser_text: String,
    state_parser_index: usize,
    char_byte_indexes: Vec<usize>,

    width: usize,
    unicode_linebreaks: Vec<(usize, BreakOpportunity)>,
    current_line: String,
    next_linebreak_index: usize,
    last_linebreak_index: usize,

    state: u8,
    current_codespan_number_of_backticks_at_start: u8,
    current_codespan_number_of_backticks_inside: u8,
}

impl MarkdownParagraphWrapper<'_> {
    pub fn new(
        text: &str,
        width: usize,
    ) -> MarkdownParagraphWrapper {
        let mut wrapper = MarkdownParagraphWrapper {
            text_iterator: text.chars().enumerate(),
            state_parser_text: text.to_string(),
            char_byte_indexes: text.char_indices()
                .map(|(index, _)| index)
                .collect::<Vec<usize>>(),
            state_parser_index: 0,

            width: width,
            unicode_linebreaks: linebreaks(text).collect::<Vec<(usize, BreakOpportunity)>>(),
            current_line: String::new(),
            next_linebreak_index: 0,
            last_linebreak_index: 0,

            state: 1,
            current_codespan_number_of_backticks_at_start: 0,
            current_codespan_number_of_backticks_inside: 0,
        };
        wrapper.next_linebreak_index = wrapper.discover_next_linebreak_index(
            0,
        );
        wrapper
    }

    fn get_char_indexes(&mut self) -> (char, usize, usize) {
        let (index, character) = self.text_iterator.next().unwrap_or((0, '\0'));
        let byte_index = self.char_byte_indexes[index];
        if index == 0 {
            let prev_character = '\0';
        } else {
            let prev_character
        }
        return (character, index, byte_index);
    }

    fn parse_character(&mut self, character: char) {
        if self.state & INSIDE_TEXT != 0 {
            if character == '`' {
                // bitwise next state
                self.state <<= 1;
                self.current_codespan_number_of_backticks_at_start = 1;
            }
        } else if self.state & ENTERING_CODESPAN != 0 {
            if character == '`' {
                self.current_codespan_number_of_backticks_at_start += 1;
            } else {
                self.state <<= 1;
            }
        } else if self.state & INSIDE_CODESPAN != 0 {
            if character == '`' {
                self.state <<= 1;
                self.current_codespan_number_of_backticks_inside += 1;
            }
        } else if self.state & EXITING_CODESPAN != 0 {
            if character == '`' {
                self.current_codespan_number_of_backticks_inside += 1;
            } else {
                if self.current_codespan_number_of_backticks_inside ==
                self.current_codespan_number_of_backticks_at_start {
                    self.state = INSIDE_TEXT;
                    self.current_codespan_number_of_backticks_at_start = 0;
                    self.current_codespan_number_of_backticks_inside = 0;
                } else {
                    self.state = INSIDE_CODESPAN;
                    self.current_codespan_number_of_backticks_inside = 0;
                }
            }
        }
    }

    fn possible_linebreak_must_wrap(&mut self, linebreak_index: &usize) -> bool {
        let mut result = false;
        loop {
            let character = self.state_parser_text.chars().nth(self.state_parser_index).unwrap_or('\0');
            if character == '\0' {
                self.state_parser_index += 1;
                break;
            }

            let byte_index = self.char_byte_indexes[self.state_parser_index];
            self.state_parser_index += 1;
            self.parse_character(character);

            if byte_index == *linebreak_index {
                if self.state == EXITING_CODESPAN {
                    // at the end of a codespan?
                    let next_character = self.state_parser_text.chars().nth(self.state_parser_index).unwrap_or('\0');
                    self.parse_character(next_character);
                    if self.state == INSIDE_TEXT {
                        // yes, must wrap
                        result = true;
                    }
                    break;
                } else {
                    // don't wrap at '-' characters because are commonly used in links
                    if character == '-' {
                        // don't break at '-' character because is commonly used by links
                        break
                    } else if character == '!' {
                        // exclamation mark followed by square bracket
                        // is considered allowed linebreak but is
                        // a common pattern for images in Markdown
                        let next_character = self.state_parser_text.chars().nth(self.state_parser_index).unwrap_or('\0');
                        if next_character == '[' {
                            break;
                        }
                    } else if character == ']' {
                        // don't break at link text end (at '](' or '][')
                        let next_character = self.state_parser_text.chars().nth(self.state_parser_index).unwrap_or('\0');
                        if next_character == '(' || next_character == '[' {
                            break;
                        }
                    }
                    result = self.state == INSIDE_TEXT;
                    break;
                }
            }
        }
        return result;
    }

    fn discover_next_linebreak_index(&mut self, start_index: usize) -> usize {
        let mut next_possible_linebreak_index = start_index;
        let start_byte_index = self.char_byte_indexes[start_index];

        let unicode_linebreaks = self.unicode_linebreaks.clone();
        for (break_index, break_opportunity) in unicode_linebreaks.iter() {
            if break_index < &start_byte_index {
                continue;
            }

            if break_opportunity == &Mandatory {
                if break_index - self.last_linebreak_index <= self.width
                        || next_possible_linebreak_index == start_index {
                    next_possible_linebreak_index = *break_index;
                }
                break
            } else {
                if self.possible_linebreak_must_wrap(&break_index) {
                    if next_possible_linebreak_index == start_byte_index {
                        next_possible_linebreak_index = *break_index;
                    }
                    let break_string_index = self.char_byte_indexes.iter().position(|r| r == break_index).unwrap();
                    if break_string_index - self.last_linebreak_index <= self.width {
                        next_possible_linebreak_index = *break_index;
                    } else {
                        break;
                    }
                }
            }
        }

        next_possible_linebreak_index
    }

    pub fn wrap(&mut self, width: usize) -> String {
        let (first_line, needs_newline) = self.next().unwrap_or(String::new());
        println!("lineeeeee: '{}'", first_line);

        result.push_str(&first_line);
        result.push_str("\n");
        wrapper.width = width;
        for line in wrapper {
            result.push_str(&line);
            println!("lineeeeee: '{}'", line);
            result.push_str("\n");
        }
        result
    }
}

impl Iterator for MarkdownParagraphWrapper<'_> {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let (character, index, byte_index) = self.get_char_indexes();
        
        if character == '\0' {
            if self.next_linebreak_index > 0 {
                self.next_linebreak_index = 0;
                return Some(self.current_line.clone());
            }
            return None;
        }
        if self.next_linebreak_index == 0 {
            // We need to discover the next linebreak index
            self.next_linebreak_index = self.discover_next_linebreak_index(
                index,
            );
        }
        
        if byte_index == self.next_linebreak_index {
            if character != '\n' && character != ' ' {
                self.current_line.push(character);
            } 
            self.last_linebreak_index = self.next_linebreak_index;
            self.state_parser_index = self.last_linebreak_index;
            self.next_linebreak_index = 0;
            let result = self.current_line.clone();

            self.current_line = String::new();
            
            return Some(result);
        } else {
            self.current_line.push(character);
            return self.next();
        }
    }
}

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
    /*
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
        // don't at strong spans
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
        // don't at italic spans
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
        // space returns empty string
        &" ",
        1,
        "",
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
        // return: 'parámetro d\ne ancho d\ne'
        &"parámetro de ancho de",
        10,
        "parámetro\nde ancho de",
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
        "",
    )]
    */
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

    /*
    #[rstest]
    #[case(
        &"aa b cc dd",
        2,
        4,
        "aa b\ncc\ndd",
    )]
    #[case(
        &"aa b cc dd ee",
        2,
        7,
        "aa b cc\ndd\nee",
    )]
    #[case(
        &"aa b cc dd ee",
        2,
        8,
        "aa b cc\ndd\nee",
    )]
    #[case(
        &"aa b cc dd ee",
        2,
        10,
        "aa b cc dd\nee",
    )]
    fn ulb_wrap_paragraph_first_line_width_test(
        #[case] text: &str,
        #[case] width: usize,
        #[case] first_line_width: usize,
        #[case] expected: String,
    ) {
        assert_eq!(
            ulb_wrap_paragraph(text, width, first_line_width),
            expected,
        );
    }
    */
}


/*
"\nf"
[(1, true), (2, true)]

-> iter 1:
           "\nf"
            --
\0 <- prev character
\n <- character
0  <- string index
0  <- byte index
1  <- next linebreak index

"\n" <- result
-------------------------

-> iter 2:
           "\nf"
              -
\n <- prev character
f  <- character
1  <- string index
2  <- byte index
1  <- next linebreak index

In this second iteration the linebreak index is reach (1).
The newline joiner must not be appended in this case.
We must compare if the previous character has been a newline.
If that is the case, we dont't need a newline joiner. 

------

if byte index == next linebreak index {

} else {
    result.push_str(character);
}


*/