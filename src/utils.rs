use tree_sitter::{Node, Language, Parser};

pub(crate) fn get_ts_parser(language: Language) -> Parser {
    let mut parser = Parser::new();
    parser.set_language(language).unwrap();
    parser
}

pub(crate) fn trace_node(node: &Node, name: &str, source_code: &str) {
    let range = node.range();
    let text = &source_code[range.start_byte..range.end_byte];
    let start_line = range.start_point.row;
    let end_line = range.end_point.row;
    let start_col = range.start_point.column;
    let end_col = range.end_point.column;
    tracing::trace!(
        "[{name}: {start_line}:{start_col} - {end_line}:{end_col}] source code:\n `{text}`\nsexp: {}",
        node.to_sexp()
    );
}
