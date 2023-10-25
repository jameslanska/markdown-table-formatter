#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/README.md"))]

mod ast;
mod table_formatter;

use table_formatter::format;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Format the GitHub Flavored Markdown tables in the `doc` string.
#[cfg(not(target_arch = "wasm32"))]
pub fn format_tables<T: AsRef<str>>(doc: T) -> String {
    format(doc)
}

/// Format the GitHub Flavored Markdown tables in the `doc` string.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn format_tables(doc: String) -> String {
    format(doc)
}
