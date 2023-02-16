# md-ulb-pwrap

Markdown paragraph wrapper using [Unicode Line Breaking
Algorithm]. Includes a Rust library with Python bindings.

Wrap a Markdown paragraph using a maximum desired width. Only works for paragraphs without other [container blocks].

Respects the prohibition against wrapping text inside
inline code blocks and other tweaks for Markdown inlines
syntax.

## Rust library

```bash
cargo add md-ulb-pwrap
```

```rust
use md_ulb_pwrap::{ulb_wrap_paragraph};

assert_eq!(
    ulb_wrap_paragraph(
        &"aaa ``` ``  ` a b c ``` ccc",
        3,
        3,
    ),
    "aaa\n``` ``  ` a b c ```\nccc",
);
```

## Python bindings

```bash
pip install md-ulb-pwrap
```

```python
from md_ulb_pwrap import ulb_wrap_paragraph

markdown = "aaa ``` ``  ` a b c ``` ccc"
expected_result = "aaa\n``` ``  ` a b c ```\nccc"
assert modify_headings_offset(markdown, 3, 3) == expected_result
```

## Reference

**ulb_wrap_paragraph**(text: *str*, width: *int*) -> *str*

- **text** (*str*): The text to wrap.
- **width** (*int*): The maximum width of the text.
- **first_line_width** (*int*): The maximum width of the first line.

**Returns** (*str*): The wrapped text.

[Unicode Line Breaking Algorithm]: https://unicode.org/reports/tr14/
[container blocks]: https://spec.commonmark.org/0.30/#container-blocks

