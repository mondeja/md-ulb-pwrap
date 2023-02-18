use crate::codespan_parser::{MarkdownCodespanParser};

use unicode_linebreak::{
    linebreaks,
    BreakOpportunity,
    BreakOpportunity::{Mandatory},
};

#[derive(Debug)]
pub struct MarkdownParagraphWrapper {
    width: usize,

    characters: Vec<(usize, (usize, char))>,
    characters_i: usize,
    linebreaks: Vec<(usize, BreakOpportunity)>,
    linebreaks_i: usize,
    codespan_parser: MarkdownCodespanParser,
    last_character_i: usize,

    next_linebreak: (usize, BreakOpportunity),
    last_linebreak_i: usize,
    current_line: String,
}

impl MarkdownParagraphWrapper {
    pub fn new(text: &str, first_line_width: usize) -> Self {
        let mut wrapper = MarkdownParagraphWrapper {
            width: first_line_width,
            
            characters: text.char_indices().enumerate().collect(),
            characters_i: 0,
            linebreaks: linebreaks(text).collect(),
            linebreaks_i: 0,
            codespan_parser: MarkdownCodespanParser::new(),
            last_character_i: text.len(),
                
            next_linebreak: (0, Mandatory),
            last_linebreak_i: 0,
            current_line: String::new(),
        };
        wrapper.update_next_linebreak();
        println!("wrapper: {:?}", wrapper);
        wrapper
    }

    fn is_linebreak_possible(
            &mut self,
            linebreak: (usize, BreakOpportunity),
    ) -> bool {
        let mut result = false;
        loop {
            let (index, (bindex, character)) = self.characters[self.codespan_parser.characters_i];
            self.codespan_parser.parse_character(character);

            if bindex + 1 == linebreak.0 {
                // reached next linebreak index
                if linebreak.1 == Mandatory {
                    // is inside text?
                    result = self.codespan_parser.context & 1 != 0;
                } else {
                    result = self.codespan_parser.context & 1 != 0;
                }
                break;
            }
        }
        result
    }

    fn update_next_linebreak(&mut self) {
        while self.linebreaks_i < self.linebreaks.len() {
            let (lb_i, lb_opp) = self.linebreaks[self.linebreaks_i];
            if self.is_linebreak_possible((lb_i, lb_opp)) {
                self.linebreaks_i += 1;
                if lb_opp == Mandatory {
                    self.next_linebreak = (lb_i, lb_opp);
                    break;
                } else {
                    self.next_linebreak = (lb_i, lb_opp);
                    // Get next linebreak index to see if we can fit more text
                    // in the line
                    let mut next_lb = self.next_linebreak;
                    loop {
                        let current_line_width = 
                            self.get_next_linebreak_index()
                            - self.last_linebreak_i;
                        if current_line_width > self.width {
                            break;
                        }
                        next_lb = self.linebreaks[self.linebreaks_i];
                        self.linebreaks_i += 1;
                    }
                    self.next_linebreak = next_lb;
                    break;
                }
            } else {
                self.linebreaks_i += 1;
            }
        }
    }

    fn get_next_linebreak_index(&mut self) -> usize {
        // Store previous state to reset at end
        let initial_linebreaks_i = self.linebreaks_i;
        self.codespan_parser.backup_state();
        
        while self.linebreaks_i < self.linebreaks.len() {
            let (lb_i, lb_opp) = self.linebreaks[self.linebreaks_i];
            if self.is_linebreak_possible((lb_i, lb_opp)) {
                break;
            } else {
                self.linebreaks_i += 1;
            }
        }
        let result = self.codespan_parser.characters_i;

        // Reset state
        self.linebreaks_i = initial_linebreaks_i;
        self.codespan_parser.restore_state();
        
        return result;
    }

    pub fn wrap(&mut self, width: usize) -> String {
        let mut result = String::new();
        let first_line = self.next().unwrap_or(String::new());
        result.push_str(&first_line);
        self.width = width;
        for line in self {
            result.push_str(&line);
        }
        result
    }
}

impl Iterator for MarkdownParagraphWrapper {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.characters_i == self.last_character_i {
            self.characters_i += 1;
            //
            // The last character have been processed
            //
            // unicode-linebreak always include a mandatory
            // linebreak at the end of the string, so we
            // return here the current line
            //
            return Some(self.current_line.clone());
            //
            // Note that the variable `characters_i` has been
            // incremented, so the next call to `next()` will
            // return `None`, stopping the iterator
            //
        } else if self.characters_i > self.last_character_i {
            return None;
        }

        let (index, (bindex, character)) = self.characters[self.characters_i];
        self.characters_i += 1;
        //println!("index: {}, bindex: {}, character: '{}'", index, bindex, character);

        if bindex == self.next_linebreak.0 {
            // reached next linebreak
            self.last_linebreak_i = index - 1;
            
            let mut result = self.current_line.clone();
            if self.next_linebreak.1 != Mandatory {
                // non mandatory linebreaks must include
                // the linebreak character
                result = result.trim_end().to_string();
                result.push('\n');
            }

            self.current_line.clear();
            self.current_line.push(character);
            
            self.update_next_linebreak();

            return Some(result);
        } else {
            self.current_line.push(character);
            return self.next();
        }
    }
}
