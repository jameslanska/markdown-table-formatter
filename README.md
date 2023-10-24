# Markdown Table Formatter

[![Rust](https://github.com/jameslanska/markdown-table-formatter/actions/workflows/rust.yml/badge.svg)](https://github.com/jameslanska/markdown-table-formatter/actions/workflows/rust.yml)[![Latest Version](https://img.shields.io/crates/v/markdown-table-formatter.svg)](https://crates.io/crates/markdown-table-formatter) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](code_of_conduct.md)

A performant table formatter fully compliant with the [Unicode 15.1.0](https://www.unicode.org/versions/Unicode15.1.0/) and [Github Flavored Markdown](https://github.github.com/gfm/) (GFM) specifications.  The formatter operates on an abstract syntax tree to fully capture the complexity of Unicode and the GFM specification.

```rust
use markdown_table_formatter::format_tables;

let unformatted = "# Example

| A | B | C |
| :-- | :--: | ----: |
| C | D | E |";

let correct = "# Example

| A    |   B   |    C |
| :--- | :---: | ---: |
| C    |   D   |    E |";

assert_eq!(format_tables(unformatted), correct);
```

Markdown Table Formatter [correctly handles](https://github.com/jameslanska/unicode-display-width) double width *grapheme clusters* such as emojis (🦀🤯) and ideographic CJK (Chinese, Japanese, Korean) characters according to [Unicode Standard Annex \#11](https://www.unicode.org/reports/tr11/) and [Unicode Technical Standard \#51](https://www.unicode.org/reports/tr51/).

Since tables are discovered in the text by parsing the document into an [abstract syntax tree](https://github.com/kivikakk/comrak), tables wrapped in HTML blocks or code blocks are ignored.  If it doesn't render, it won't be formatted.

The goal of this project is to define a *correct* base implementation that can be used in other projects.  If you discover a correctness error or other bug, please [open a GitHub issue](CONTRIBUTING.md).

## Alignment Issues

If the table alignment looks slightly off and you are unsure if the formatting is correct, open the file in vim to check.  The following snippet may not align perfectly in VS Code or the GitHub Markdown render, but it will align correctly in vim.

```markdown
| A   | B   |
| --- | --- |
| ✅  | ❌  |
```

Many text editors such as VS Code do not use true monospacing.  For a more in-depth discussion, see the [Fonts](https://github.com/jameslanska/unicode-display-width/blob/main/docs/fonts.md) documentation section from Markdown Table Formatter's sister project, [Unicode Display Width](https://github.com/jameslanska/unicode-display-width).

## Acknowledgements

I would like to express my deep and sincere gratitude to [Joe Lanska](https://github.com/josephlanska) for his unwavering support and for all the time he spent helping me improve the documentation.

## Support

If you would like to support further development, please consider [buying me a coffee](https://www.buymeacoffee.com/lanskajames).
