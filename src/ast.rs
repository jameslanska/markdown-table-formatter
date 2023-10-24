//! This module includes all functionality to find the location of tables inside a document.
//!
//! This is a deceptively hard problem that cannot be expressed correctly in a concise regular expression.  For example, not every row will have a vertical bar in it, and tables can be nested within blockquotes as long as the blockquote indentation is consistent.
//!
//! As of September 2023, there are three main markdown parsers in the Rust ecosystem: pulldown-cmark, comrak, and markdown-rs.
//!
//! Pulldown-cmark doesn't offer native access to an AST.  `markdown-rs` is still in alpha development, and its table parsing isn't correct as of October 2023.
//!
//! `comrak` is the only crate (that I know of) that implements table parsing exactly to spec, but it has significant limitations.  AST nodes do not include byte indexes on the original string.  This makes correctly processing individual elements very difficult.  
//!
//! Converting a Markdown table to its AST representation for future rendering to HTML is a lossy operation.  If there are only 3 alignment cells in the delimiter row, but there are 4 cells of content on a future row, the AST parsed by `comrak` will not include the last cell.  This is perfectly acceptable for HTML rendering because that last cell ins't rendered according to the GFM specification.  However, a table formatter should never delete information, only manipulate whitespace.  This module uses the AST to get line numbers for the tables.  Then all text on that line range is extracted and formatted.
//!
//! While comrak does include parsing of table rows and cells, this module manually processes cells to ensure that no data is lost and that no indexing errors occur on strange unicode characters.

use comrak::{
    arena_tree::Children,
    nodes::{Ast, AstNode, NodeValue, TableAlignment},
    parse_document, Arena, ComrakOptions, ComrakParseOptions, ComrakRenderOptions,
    ExtensionOptions,
};
use std::ops::{Range, RangeInclusive};

/// Parse the document, populate the `arena` variable with the nodes, and return the root node of the AST.
///
/// All optional parse options not related to GitHub Flavored Markdown tables are turned off.
fn get_ast<'ast>(arena: &'ast Arena<AstNode<'ast>>, doc: &str) -> &'ast AstNode<'ast> {
    // The returned nodes are created in the supplied Arena, and are bound by its lifetime.

    let mut extension_options = ExtensionOptions::default().clone();
    extension_options.strikethrough = false;
    extension_options.tagfilter = false;
    extension_options.table = true;
    extension_options.autolink = false;
    extension_options.tasklist = false;
    extension_options.superscript = false;
    extension_options.header_ids = None;
    extension_options.footnotes = false;
    extension_options.description_lists = false;
    extension_options.front_matter_delimiter = None;

    let options = ComrakOptions {
        extension: extension_options,
        parse: ComrakParseOptions::default(),
        render: ComrakRenderOptions::default(),
    };

    parse_document(arena, doc, &options)
}

/// Return all table AST nodes along with the table alignment vector
fn search_children(
    nodes: Children<'_, std::cell::RefCell<Ast>>,
) -> Vec<(Vec<TableAlignment>, &AstNode<'_>)> {
    let mut tables: Vec<(Vec<TableAlignment>, &AstNode<'_>)> = vec![];

    for node in nodes {
        match &node.data.borrow().value {
            NodeValue::Document => unreachable!("This only appears at the tree root"),
            NodeValue::FrontMatter(_) => unreachable!(), // `front_matter_delimeter` is `None` in `extension_options`
            NodeValue::BlockQuote => tables.append(&mut search_children(node.children())),
            NodeValue::List(_) => (), // tables shouldn't ever be in a list
            NodeValue::Item(_) => (), // tables shouldn't ever be in a list item
            NodeValue::DescriptionList => {
                unreachable!("DescriptionLists is `false` in `extension_options`")
            }
            NodeValue::DescriptionItem(_) => {
                unreachable!("DescriptionLists is `false` in `extension_options`")
            }
            NodeValue::DescriptionTerm => {
                unreachable!("DescriptionLists is `false` in `extension_options`")
            }
            NodeValue::DescriptionDetails => {
                unreachable!("DescriptionLists is `false` in `extension_options`")
            }
            NodeValue::CodeBlock(_) => (), // tables inside a code block are ignored by the parser, so no table can be found inside this node
            NodeValue::HtmlBlock(_) => (), // tables inside an HTML block are ignored by the parser, so no table can be found inside this node
            NodeValue::Paragraph => (),    // tables cannot be nested in a paragraph
            NodeValue::Heading(_) => (), // tables cannot be nested in a heading since they require multiple lines
            NodeValue::ThematicBreak => (), // has no children
            NodeValue::FootnoteDefinition(_) => {
                unreachable!("`footnotes` are not turned on in `extension_options`")
            }
            NodeValue::Table(table_metadata) => {
                tables.push((table_metadata.alignments.clone(), node))
            }
            NodeValue::TableRow(_) => (), // we are only interested in the top level table
            NodeValue::TableCell => (),   // we are only interested in the top level table
            NodeValue::Text(_) => (),     // inline item
            NodeValue::TaskItem(_) => (), // inline item
            NodeValue::SoftBreak => (),   // inline item
            NodeValue::LineBreak => (),   // does not have nested items
            NodeValue::Code(_) => (),     // inline item
            NodeValue::HtmlInline(_) => (), // inline item
            NodeValue::Emph => (),        // inline item
            NodeValue::Strong => (),      // inline item
            NodeValue::Strikethrough => (), // inline item
            NodeValue::Superscript => (), // inline item
            NodeValue::Link(_) => (),     // inline item
            NodeValue::Image(_) => (),    // inline item
            NodeValue::FootnoteReference(_) => {
                unreachable!("`footnotes` are not turned on in `extension_options`")
            }
        }
    }

    tables
}

/// All necessary information to format a table
pub struct TableInDocument<'a> {
    pub range: Range<usize>,
    pub text: &'a str,
    pub alignments: Vec<TableAlignment>,
}

/// Get the byte offset range of `doc` that corresponds to the inclusive range of lines.
fn get_range_from_lines(doc: &str, lines: RangeInclusive<usize>) -> Range<usize> {
    let mut line = 1;
    let mut line_start = true;
    let mut start_offset = 0;

    for (index, character) in doc.char_indices() {
        if character == '\n' {
            if line == *lines.end() {
                // CRLF
                if doc.get(index - 1..index) == Some("\r") {
                    return start_offset..index - 1;
                }

                return start_offset..index;
            }

            line += 1;
            line_start = true;
        } else {
            if line_start && line == *lines.start() {
                start_offset = index;
            }

            line_start = false;
        }
    }

    start_offset..doc.len()
}

/// Use Comrak abstract syntax tree to find the locations (and alignments) of all GitHub Flavored Markdown tables in the `doc` string.
pub fn get_tables(doc: &str) -> Vec<TableInDocument> {
    let arena: Arena<AstNode<'_>> = Arena::new();
    let ast: &AstNode<'_> = get_ast(&arena, doc);

    let mut tables: Vec<TableInDocument> = vec![];

    for (alignments, table_ast) in search_children(ast.children()) {
        let position = table_ast.data.borrow().sourcepos;

        let lines: RangeInclusive<usize> = position.start.line..=position.end.line;
        let range = get_range_from_lines(doc, lines);

        let text = doc
            .get(range.clone())
            .expect("This should always be at a valid index since we are iterating over chars.");

        tables.push(TableInDocument {
            range,
            text,
            alignments,
        });
    }

    tables
}
