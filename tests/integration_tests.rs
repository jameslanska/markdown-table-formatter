use markdown_table_formatter::format_tables;
use test_case::test_case;

const BASIC_TABLE: &str = include_str!("text_snippets/basic_table.txt");
const BASIC_TABLE_FIXED: &str = include_str!("text_snippets/basic_table_fixed.txt");
const TABLE_ALIGNMENT: &str = include_str!("text_snippets/table_alignment.txt");
const TABLE_ALIGNMENT_FIXED: &str = include_str!("text_snippets/table_alignment_fixed.txt");
const TABLE_IN_CODE_BLOCK: &str = include_str!("text_snippets/table_in_code_block.txt");
const TABLE_IN_HTML_COMMENT: &str = include_str!("text_snippets/table_in_html_comment.txt");
const TABLE_WIDTH_EXPANSION: &str = include_str!("text_snippets/table_width_expansion.txt");
const TABLE_WIDTH_EXPANSION_FIXED: &str =
    include_str!("text_snippets/table_width_expansion_fixed.txt");
const TABLE_WIDTH_REDUCTION: &str = include_str!("text_snippets/table_width_reduction.txt");
const TABLE_WIDTH_REDUCTION_FIXED: &str =
    include_str!("text_snippets/table_width_reduction_fixed.txt");
const TABLE_WITH_EMPTY_CELLS: &str = include_str!("text_snippets/table_with_empty_cells.txt");
const TABLE_WITH_EMPTY_CELLS_FIXED: &str =
    include_str!("text_snippets/table_with_empty_cells_fixed.txt");
const TABLE_WITH_PIPE_IN_CODE_BLOCK: &str =
    include_str!("text_snippets/table_with_pipe_in_code_block.txt");
const TABLE_WITH_PIPE_IN_CODE_BLOCK_FIXED: &str =
    include_str!("text_snippets/table_with_pipe_in_code_block_fixed.txt");
const TABLE_WITH_TRAILING_TEXT: &str = include_str!("text_snippets/table_with_trailing_text.txt");
const TABLE_WITH_TRAILING_TEXT_FIXED: &str =
    include_str!("text_snippets/table_with_trailing_text_fixed.txt");
const TWO_ROW_TABLE: &str = include_str!("text_snippets/2_row_table.txt");
const TWO_ROW_TABLE_FIXED: &str = include_str!("text_snippets/2_row_table_fixed.txt");
const TABLE_IN_BLOCKQUOTE: &str = include_str!("text_snippets/table_in_blockquote.txt");
const TABLE_IN_BLOCKQUOTE_FIXED: &str = include_str!("text_snippets/table_in_blockquote_fixed.txt");
const TABLE_WITH_WEIRD_UNICODE: &str = include_str!("text_snippets/weird_unicode.txt");
const TABLE_WITH_WEIRD_UNICODE_FIXED: &str = include_str!("text_snippets/weird_unicode_fixed.txt"); // If you are unconvinced with the test, open up the fixed file in VIM
const EMOJI_TABLE: &str = include_str!("text_snippets/emoji_table.txt");
const EMOJI_TABLE_FIXED: &str = include_str!("text_snippets/emoji_table_fixed.txt");
const INTERSPERSED_TABLES: &str = include_str!("text_snippets/interspersed_tables.txt");
const INTERSPERSED_TABLES_FIXED: &str = include_str!("text_snippets/interspersed_tables_fixed.txt");

#[test]
fn crlf_test() {
    let input = "|A|B|\r\n|-|-|\r\n|C|D|";
    let correct = "| A   | B   |\r\n| --- | --- |\r\n| C   | D   |";

    assert_eq!(format_tables(input), correct);
}

#[test_case(BASIC_TABLE, BASIC_TABLE_FIXED)]
#[test_case(TABLE_ALIGNMENT, TABLE_ALIGNMENT_FIXED)]
#[test_case(TABLE_IN_CODE_BLOCK, TABLE_IN_CODE_BLOCK)]
#[test_case(TABLE_IN_HTML_COMMENT, TABLE_IN_HTML_COMMENT)]
#[test_case(TABLE_WIDTH_EXPANSION, TABLE_WIDTH_EXPANSION_FIXED)]
#[test_case(TABLE_WIDTH_REDUCTION, TABLE_WIDTH_REDUCTION_FIXED)]
#[test_case(TABLE_WITH_EMPTY_CELLS, TABLE_WITH_EMPTY_CELLS_FIXED)]
#[test_case(TABLE_WITH_PIPE_IN_CODE_BLOCK, TABLE_WITH_PIPE_IN_CODE_BLOCK_FIXED)]
#[test_case(TABLE_WITH_TRAILING_TEXT, TABLE_WITH_TRAILING_TEXT_FIXED)]
#[test_case(TWO_ROW_TABLE, TWO_ROW_TABLE_FIXED)]
#[test_case(TABLE_IN_BLOCKQUOTE, TABLE_IN_BLOCKQUOTE_FIXED)]
#[test_case(TABLE_WITH_WEIRD_UNICODE, TABLE_WITH_WEIRD_UNICODE_FIXED)]
#[test_case(EMOJI_TABLE, EMOJI_TABLE_FIXED)]
#[test_case(INTERSPERSED_TABLES, INTERSPERSED_TABLES_FIXED)]
fn test_table_formatter(preformatted: &str, correct: &str) {
    let formatted = format_tables(preformatted);
    assert_eq!(formatted, correct);

    // idempotency test
    assert_eq!(format_tables(correct), correct);
}
