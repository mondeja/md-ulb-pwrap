// Codespan parser contexts
static INSIDE_TEXT: u8 = 0b1;
static ENTERING_CODESPAN: u8 = 0b10;
static INSIDE_CODESPAN: u8 = 0b100;
static EXITING_CODESPAN: u8 = 0b1000;

#[derive(Default)]
pub struct MarkdownWrapOpportunitiesParser {
    pub context: u8,
    current_codespan_number_of_backticks_at_start: u8,
    current_codespan_number_of_backticks_inside: u8,

    previous_character: char,
    inside_link: bool,
    inside_image_link: bool,

    pub characters_i: usize,

    states: Vec<(u8, u8, u8, char, bool, bool, usize)>,
}

impl MarkdownWrapOpportunitiesParser {
    pub fn new() -> Self {
        MarkdownWrapOpportunitiesParser {
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
            } else if self.current_codespan_number_of_backticks_inside
                == self.current_codespan_number_of_backticks_at_start
            {
                self.context = INSIDE_TEXT;
                self.current_codespan_number_of_backticks_at_start = 0;
                self.current_codespan_number_of_backticks_inside = 0;
            } else {
                self.context = INSIDE_CODESPAN;
                self.current_codespan_number_of_backticks_inside = 0;
            }
        }

        if self.previous_character == '!' {
            self.inside_image_link = character == '[';
        } else {
            self.inside_image_link = false;
            if self.previous_character == ']' {
                self.inside_link = character == '(' || character == '[';
            } else {
                self.inside_link = false;
            }
        }

        self.previous_character = character;

        self.characters_i += 1;
    }

    pub fn is_inside_text(&self) -> bool {
        self.context & INSIDE_TEXT != 0
    }

    pub fn is_inside_link(&self) -> bool {
        !self.inside_link && !self.inside_image_link
    }

    pub fn backup_state(&mut self) {
        self.states.push((
            self.context,
            self.current_codespan_number_of_backticks_at_start,
            self.current_codespan_number_of_backticks_inside,
            self.previous_character,
            self.inside_link,
            self.inside_image_link,
            self.characters_i,
        ));
    }

    pub fn restore_state(&mut self) {
        let state = self.states.pop().unwrap();

        self.context = state.0;
        self.current_codespan_number_of_backticks_at_start = state.1;
        self.current_codespan_number_of_backticks_inside = state.2;
        self.previous_character = state.3;
        self.inside_link = state.4;
        self.inside_image_link = state.5;
        self.characters_i = state.6;
    }
}
