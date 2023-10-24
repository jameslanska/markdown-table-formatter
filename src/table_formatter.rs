//! See `table_formatter_behavior.md` for a description of behavior specific to this formatter.

use comrak::nodes::TableAlignment;
use unicode_display_width::width;

use crate::ast::TableInDocument;

const INDEX_OF_DELIMITER_ROW: usize = 1;

type Row = Vec<Cell>;
struct TableRows {
    rows: Vec<Row>,
}

/// Represents a table cell.
///
/// ## Values
///
/// - `visual_length` is the number of columns required to display the `content` string in a monospace editor such as vim.
/// - `content` is the text of the cell with no leading or trailing whitespace.
///
#[derive(Clone, Debug, PartialEq, Eq)]
struct Cell {
    content: String,
    visual_length: usize,
}

impl From<Vec<char>> for Cell {
    fn from(value: Vec<char>) -> Self {
        let cell_string: String = value.into_iter().collect();
        let cell_string: String = cell_string.trim().to_string();

        Cell {
            visual_length: width(&cell_string) as usize, // no cell should exceed 2 billion columns
            content: cell_string,
        }
    }
}

/// Return the minimum number of characters a delimiter cell can have given a specific alignment, not including any whitespace padding.
///
/// The minimum alignment cell text is given in the following table:
///
/// | Alignment       | Minimum characters |
/// | :-------------- | :----------------- |
/// | left            | `:---`             |
/// | right           | `---:`             |
/// | center          | `:---:`            |
/// | not specified   | `---`              |
///
/// ## Returns
///
/// The number of characters in the delimiter row
///
fn get_alignment_cell_minimum_width(alignment: &TableAlignment) -> u8 {
    match alignment {
        TableAlignment::Center => 5,
        TableAlignment::Left | TableAlignment::Right => 4,
        TableAlignment::None => 3,
    }
}

/// Return an array of maximum widths for each column defined with an alignment cell.
///
/// ## Parameters
///
/// - `content_rows` all rows of the table except for the delimiter row
/// - `alignments`
///
/// ## Remarks
///
/// If there are cells with a column number greater than the number of alignment cells in the delimiter row, they are ignored by this function.
///
/// The second row (delimiter row) has special consideration.  The number of hyphens of each cell as it exists in the pre-formatted table is ignored.  Instead a minimum size is given depending on the orientation, so that it has at least 3 hyphens.  This enables the table to shrink as appropriate.  As such, only the *content* rows should be passed to this function.
///
/// The result of this function is used to set the amount of whitespace padding to ensure each entry has the same width in each column.
///
/// The maximum width does not include any leading or trailing whitespace.
///
fn get_col_max_widths(content_rows: &[Row], alignments: &[TableAlignment]) -> Vec<usize> {
    let mut max_widths: Vec<usize> = Vec::new();

    for (index, _) in alignments.iter().enumerate() {
        max_widths.push(
            content_rows
                .iter()
                .filter(|row| row.get(index).is_some())
                .map(|row| row[index].visual_length)
                .max()
                .unwrap_or(0)
                .max(get_alignment_cell_minimum_width(&alignments[index]) as usize),
        );
    }

    max_widths
}

/// Get the formatted string to place in an delimiter cell.
///
/// ## Parameters
///
/// - `alignment`
/// - `width` visual width of the cell not including leading or trailing whitespace
///
fn format_delimiter_cell(alignment: &TableAlignment, width: usize) -> String {
    match alignment {
        TableAlignment::Center => format!(" :{}: ", String::from("-").repeat(width - 2)),
        TableAlignment::Left => format!(" :{} ", String::from("-").repeat(width - 1)),
        TableAlignment::Right => format!(" {}: ", String::from("-").repeat(width - 1)),
        TableAlignment::None => format!(" {} ", String::from("-").repeat(width)),
    }
}

/// Return an array of strings with the delimiter cells normalized to align with maximum column widths.  Unlike all cells not in a delimiter cell, this is achieved by adding or removing dashes to match the maximum width of each column.
fn get_normalized_delimiter_row(
    column_alignment: &[TableAlignment],
    column_max_widths: &[usize],
) -> Vec<String> {
    if column_alignment.len() != column_max_widths.len() {
        panic!(
            "The length of the `column_alignment` and `column_max_widths` vectors must be equal."
        );
    }

    std::iter::zip(column_alignment.iter(), column_max_widths.iter())
        .map(|(alignment, width)| format_delimiter_cell(alignment, *width))
        .collect()
}

/// Add whitespace as necessary according to `align`
///
/// ## Parameters
///
/// - `cell`
/// - `length` max visual length of any element (not including any leading or trailing whitespace)
/// - `align`
///
/// ## Returns
///
/// String that
///
/// - is aligned according to `align`,
/// - has at least one space on each end, and
/// - has a visual length of 2 greater than `length`.
///
fn align_cell(cell: &Cell, align: &TableAlignment, length: &usize) -> String {
    if cell.visual_length > *length {
        panic!(
            "Invalid `length` argument.  It must be greater than or equal to `cell.visual_length`"
        )
    }

    match align {
        TableAlignment::Center if *length > cell.visual_length => {
            // Integer division truncates *toward zero* to the nearest integer.
            let leading_whitespace: usize = (length - cell.visual_length) / 2;
            let trailing_whitespace = length - cell.visual_length - leading_whitespace;

            let leading_whitespace = String::from(" ").repeat(leading_whitespace);
            let trailing_whitespace = String::from(" ").repeat(trailing_whitespace);

            format!(
                " {0}{1}{2} ",
                leading_whitespace, cell.content, trailing_whitespace
            )
        }
        TableAlignment::Right => {
            format!(
                " {0}{1} ",
                String::from(" ").repeat(length - cell.visual_length),
                cell.content
            )
        }
        _ => {
            format!(
                " {0}{1} ",
                cell.content,
                String::from(" ").repeat(length - cell.visual_length)
            )
        }
    }
}

/// Return the text preceding the start of the table on its first line.
///
/// ## Remarks
///
/// Tables can be nested inside one or more blockquotes so long as the blockquote indent is consistent for the entire table.
///
/// ## Returns
///
/// If the table starts at the beginning of the line, return `None`.  Otherwise, return a String composed of spaces and `>`.
///
fn get_table_indentation(table_header: &str) -> Option<String> {
    let mut indentation = String::new();

    let allowed_chars = [' ', '>'];

    for character in table_header.chars() {
        if allowed_chars.contains(&character) {
            indentation.push(character);
        } else {
            break;
        }
    }

    if indentation.is_empty() {
        None
    } else {
        Some(indentation)
    }
}

/// Return owned `TableRow` structs with owned data inside that represent the contents of each row.
///
/// The delimiter row is ignored by this function as that is parsed by `comrak` and can be regenerated based on the alignment values.
fn get_table_content_rows(table: &str) -> TableRows {
    let content_row_iter = table.lines().take(1).chain(table.lines().skip(2));
    let rows = content_row_iter.map(parse_row_text).collect();

    TableRows { rows }
}

/// Format a GFM table.
///
/// - each cell has at least one space at the start and end.
/// - each cell has the same visual width as each of the other cells in its column if that column has an alignment cell
/// - no text is ever deleted except for leading and trailing whitespace in a cell
/// - will not panic on malformed tables (any panic is a bug)
pub fn format_table(table: &TableInDocument<'_>) -> String {
    let table_rows = get_table_content_rows(table.text);

    // Column "content" width (the length of the longest cell in each column), **without padding**
    let column_max_widths: Vec<usize> = get_col_max_widths(&table_rows.rows, &table.alignments);

    let delimiter_row = get_normalized_delimiter_row(&table.alignments, &column_max_widths);
    let delimiter_row: String = format!("|{}|", delimiter_row.join("|"));

    // align all the cells
    let mut rows: Vec<String> = Vec::new();
    for row in table_rows.rows {
        let mut aligned_cells: Vec<String> = Vec::new();
        for (index, cell) in row.iter().enumerate() {
            let align: &TableAlignment = match &table.alignments.get(index) {
                Some(val) => val,
                None => &TableAlignment::None,
            };

            let length: usize = match column_max_widths.get(index) {
                Some(val) => *val,
                None => width(&cell.content) as usize,
            };

            aligned_cells.push(align_cell(cell, align, &length));
        }

        let row: String = format!("|{}|", aligned_cells.join("|"));
        rows.push(row);
    }

    rows.insert(INDEX_OF_DELIMITER_ROW, delimiter_row);

    if let Some(indentation) = get_table_indentation(table.text.lines().next().unwrap()) {
        rows = rows
            .into_iter()
            .map(|row| format!("{indentation}{row}"))
            .collect();
    }

    const CRLF: &str = "\r\n";
    if table.text.contains(CRLF) {
        return rows.join(CRLF);
    }

    rows.join("\n")
}

/// The GFM specification does not distinguish between vertical bars in code blocks or regular vertical bars.  Even in code blocks, they need to be escaped.  Given this, the formatting behavior of this project may look strange.
fn parse_row_text(line: &str) -> Vec<Cell> {
    // remove any leading whitespace and all blockquote nesting
    let line = line.trim_matches(|c| [' ', '>'].contains(&c));

    let mut previous_char_was_backslash = false;
    let mut char_iter = line.chars();

    if line.is_empty() || line == "|" {
        return vec![];
    }

    // normalize
    if line.starts_with('|') {
        // ignore the first value
        char_iter.next();
    }

    let mut cells: Vec<Cell> = Vec::new();
    let mut cell: Vec<char> = Vec::new();

    for scalar_value in char_iter {
        if scalar_value == '|' && !previous_char_was_backslash {
            cells.push(cell.into());
            cell = Vec::new();
        } else {
            if scalar_value == '\\' {
                // allow multiples of two backslashes to cancel each other out
                previous_char_was_backslash = !previous_char_was_backslash;
            } else {
                previous_char_was_backslash = false;
            }
            cell.push(scalar_value);
        }
    }

    if !line.ends_with('|') {
        cells.push(cell.into());
    }

    cells
}

#[cfg(test)]
mod test {
    use super::*;
    use test_case::test_case;

    #[test_case(
        "| a | b | c |",
        &["a", "b", "c"];
        "Simple row"
    )]
    #[test_case(
        "a | b | c",
        &["a", "b", "c"];
        "No leading or trailing |"
    )]
    #[test_case(
        "| a | b | `\\|` |",
        &["a", "b", "`\\|`"];
        "With escaped vertical bar in code block"
    )]
    #[test_case(
        "| a | b | \\| |",
        &["a", "b", "\\|"];
        "With escaped vertical bar"
    )]
    #[test_case(
        "| a | b | `|` |",
        &["a", "b", "`", "`"];
        "Vertical bar in code block without escape"
    )]
    #[test_case(
        "a", 
        &["a"]; 
        "Single element"
    )]
    #[test_case(
        "| --- | :---: | ---: | :--- |",
        &["---", ":---:", "---:", ":---"];
        "Delimiter row"
    )]
    #[test_case(
        "| ✅ | ❌ |",
        &["✅", "❌"];
        "Non-ASCII text"
    )]
    #[test_case(
        "| | D",
        &["", "D"];
        "Empty cell"
    )]
    #[test_case(
        "   > >>  ✅ | ❌",
        &["✅", "❌"];
        "Blockquote indentation"
    )]
    fn test_row_separation(input: &str, output: &[&str]) {
        let correct_row: Row = output
            .iter()
            .map(|el| Cell {
                content: el.to_string(),
                visual_length: width(el) as usize,
            })
            .collect();
        let cells = parse_row_text(input);
        assert_eq!(cells, correct_row);
    }
}
