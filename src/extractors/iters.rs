use tree_sitter::{Language, Node, Query, QueryCursor};

pub(crate) fn get_iters<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<Node<'a>> {
    // FIXME(lsp): find all function calls and filter by those that return iterators (maybe ones that aren't passed an iterator either.
    let query = Query::new(
        language,
        "(call_expression
            function: (field_expression
                field: (field_identifier) @fn_name
                 (#match? @fn_name \"iter|iter_mut|into_iter\")
         ))",
    )
    .unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .flat_map(|matched| matched.captures.iter().map(|cap| cap.node))
        .collect()
}
