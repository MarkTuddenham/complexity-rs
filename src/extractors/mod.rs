pub(crate) mod matches;
pub(crate) mod ifs;
pub(crate) mod loops;
pub(crate) mod iters;

use tree_sitter::{Language, Node, Query, QueryCursor};

pub(crate) fn get_functions<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<(&'a str, Node<'a>)> {
    let query = Query::new(
        language,
        "(function_item
             name: (identifier) @fn_name
             body: (_) @fn_body
         )",
    )
    .unwrap();

    let body_idx = query.capture_index_for_name("fn_body").unwrap();
    let name_idx = query.capture_index_for_name("fn_name").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .flat_map(|matched| {
            let names = matched
                .captures
                .iter()
                .filter(|c| c.index == name_idx)
                .map(|capture| {
                    let range = capture.node.range();
                    &source_code[range.start_byte..range.end_byte]
                });

            let nodes = matched
                .captures
                .iter()
                .filter(|c| c.index == body_idx)
                .map(|capture| capture.node);

            names.zip(nodes)
        })
        .collect()
}

