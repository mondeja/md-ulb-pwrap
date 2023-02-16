from unicode_linebreak import linebreaks

INSIDE_TEXT = 1
ENTERING_CODESPAN = 2
INSIDE_CODESPAN = 4
EXITING_CODESPAN = 8


class MarkdownParagraphWrapper:
    def __init__(self, text: str, width: int) -> None:
        self.text_last_index = len(text) - 1
        self.text = text
        self.text_iterator_index = -1
        self.state_parser_index = 0

        self.width = width
        self.unicode_linebreaks = [
            (index - 1, break_opportunity)
            for index, break_opportunity in linebreaks(text)
        ]
        self.current_line = ""
        self.next_linebreak_index = 0
        self.last_linebreak_index = 0

        self.state = 1
        self.current_codespan_number_of_backticks_at_start = 0
        self.current_codespan_number_of_backticks_inside = 0

    def parse_character(self, character: str) -> None:
        if self.state == INSIDE_TEXT:
            if character == "`":
                self.state <<= 1
                self.current_codespan_number_of_backticks_at_start = 1
        elif self.state == ENTERING_CODESPAN:
            if character == "`":
                self.current_codespan_number_of_backticks_at_start += 1
            else:
                self.state <<= 1
        elif self.state == INSIDE_CODESPAN:
            if character == "`":
                self.state <<= 1
                self.current_codespan_number_of_backticks_inside += 1
        elif self.state == EXITING_CODESPAN:
            if character == "`":
                self.current_codespan_number_of_backticks_inside += 1
            else:
                if (
                    self.current_codespan_number_of_backticks_inside
                    == self.current_codespan_number_of_backticks_at_start
                ):
                    self.state = INSIDE_TEXT
                    self.current_codespan_number_of_backticks_at_start = 0
                    self.current_codespan_number_of_backticks_inside = 0
                else:
                    self.state = INSIDE_CODESPAN
                    self.current_codespan_number_of_backticks_inside = 0

    def possible_linebreak_must_wrap(self, linebreak_index: int) -> bool:
        result = False
        while True:
            try:
                character = self.text[self.state_parser_index]
            except IndexError:
                break
            self.state_parser_index += 1
            self.parse_character(character)
            if self.state_parser_index == linebreak_index:
                if self.state == EXITING_CODESPAN:
                    try:
                        next_character = self.text[self.state_parser_index]
                    except IndexError:
                        break
                    self.parse_character(next_character)
                    if self.state == INSIDE_TEXT:
                        result = True
                    break
                else:
                    next_character = self.text[self.state_parser_index]
                    if next_character == "-":
                        break
                    elif next_character == "!":
                        next2_character = self.text[self.state_parser_index + 1]
                        if next2_character == "[":
                            break
                    elif next_character == "]":
                        next2_character = self.text[self.state_parser_index + 1]
                        if next2_character in ("(", "["):
                            break
                    result = self.state == INSIDE_TEXT
                    break
        return result

    def discover_next_linebreak_index(self, start_index: int) -> int:
        next_possible_linebreak_index = start_index

        for break_index, break_opportunity_is_mandatory in self.unicode_linebreaks:
            if break_index < start_index:
                continue

            if break_opportunity_is_mandatory:
                if (
                    break_index - self.last_linebreak_index <= self.width
                ) or next_possible_linebreak_index == start_index:
                    next_possible_linebreak_index = break_index
                break
            elif self.possible_linebreak_must_wrap(break_index):
                if next_possible_linebreak_index == start_index:
                    next_possible_linebreak_index = break_index
                if break_index - self.last_linebreak_index <= self.width:
                    next_possible_linebreak_index = break_index
                else:
                    break

        return next_possible_linebreak_index

    def __iter__(self):
        return self

    def __next__(self):
        self.text_iterator_index += 1
        try:
            character = self.text[self.text_iterator_index]
        except IndexError:
            raise StopIteration

        if self.next_linebreak_index == 0:
            self.next_linebreak_index = self.discover_next_linebreak_index(
                self.text_iterator_index,
            )

        if self.text_iterator_index == self.next_linebreak_index:
            if self.text_iterator_index == self.text_last_index:
                self.current_line += character
            self.last_linebreak_index = self.next_linebreak_index
            self.state_parser_index = self.last_linebreak_index + 1
            self.next_linebreak_index = 0
            result = self.current_line
            self.current_line = ""
            return result
        else:
            self.current_line += character
            return self.__next__()


def ulb_wrap_paragraph(text: str, width: int, first_line_width: int) -> str:
    result = ""
    wrapper = MarkdownParagraphWrapper(text, first_line_width)
    wrapper.width = width
    result += f"{next(wrapper)}\n"
    for line in wrapper:
        result += f"{line}\n"

    reversed_text = text[::-1]
    for i, character in enumerate(result[::-1]):
        if character != "\n":
            break
        if reversed_text[i] != "\n":
            result = result[:-1]
    return result
