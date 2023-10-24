#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod ast;
mod table_formatter;

use ast::get_tables;
use table_formatter::format_table;

/// Format the GitHub Flavored Markdown tables in the `doc` string.
pub fn format_tables<T: AsRef<str>>(doc: T) -> String {
    let doc: &str = doc.as_ref();

    let mut fixed = String::with_capacity((doc.len() as f64 * 1.2) as usize);

    let mut last_match = 0;
    for table in &get_tables(doc) {
        let start = table.range.start;
        let end = table.range.end;

        fixed.push_str(&doc[last_match..start]);
        fixed.push_str(&format_table(table));
        last_match = end;
    }

    // last match may be the offset immediately after the end of the string.
    if last_match < doc.len() {
        fixed.push_str(&doc[last_match..]);
    }

    fixed
}
