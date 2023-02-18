// Codespan parser contexts
static INSIDE_TEXT: u8 = 0b1;
static ENTERING_CODESPAN: u8 = 0b10;
static INSIDE_CODESPAN: u8 = 0b100;
static EXITING_CODESPAN: u8 = 0b1000;

#[derive(Default, Debug)]
pub struct MarkdownCodespanParser {
    pub context: u8,
    current_codespan_number_of_backticks_at_start: u8,
    current_codespan_number_of_backticks_inside: u8,
    
    pub characters_i: usize,

    state: (u8, u8, u8, usize),
}

impl MarkdownCodespanParser {
    pub fn new() -> Self {
        MarkdownCodespanParser {
            context: 1,
            ..Default::default()
        }
    }

    pub fn parse_character(&mut self, character: char) {
        if self.context & INSIDE_TEXT != 0 {
            if character == '`' {
                // bitwise next context
                self.context <<= 1;
                self.current_codespan_number_of_backticks_at_start = 1;
            }
        } else if self.context & ENTERING_CODESPAN != 0 {
            if character == '`' {
                self.current_codespan_number_of_backticks_at_start += 1;
            } else {
                self.context <<= 1;
            }
        } else if self.context & INSIDE_CODESPAN != 0 {
            if character == '`' {
                self.context <<= 1;
                self.current_codespan_number_of_backticks_inside += 1;
            }
        } else if self.context & EXITING_CODESPAN != 0 {
            if character == '`' {
                self.current_codespan_number_of_backticks_inside += 1;
            } else {
                if self.current_codespan_number_of_backticks_inside ==
                self.current_codespan_number_of_backticks_at_start {
                    self.context = INSIDE_TEXT;
                    self.current_codespan_number_of_backticks_at_start = 0;
                    self.current_codespan_number_of_backticks_inside = 0;
                } else {
                    self.context = INSIDE_CODESPAN;
                    self.current_codespan_number_of_backticks_inside = 0;
                }
            }
        }
        self.characters_i += 1;
    }

    pub fn could_wrap(&self) -> bool {
        self.context & INSIDE_TEXT != 0
    }

    pub fn backup_state(&mut self) {
        self.state = (
            self.context,
            self.current_codespan_number_of_backticks_at_start,
            self.current_codespan_number_of_backticks_inside,
            self.characters_i,
        );
    }

    pub fn restore_state(&mut self) {
        self.context = self.state.0;
        self.current_codespan_number_of_backticks_at_start = self.state.1;
        self.current_codespan_number_of_backticks_inside = self.state.2;
        self.characters_i = self.state.3;
    }
}
