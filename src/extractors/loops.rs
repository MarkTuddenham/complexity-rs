use tree_sitter::{Language, Node, Query, QueryCursor};

pub(crate) struct ForResult<'a> {
    pub(crate) node: Node<'a>,
}

pub(crate) fn get_for_loops<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<ForResult<'a>> {
    let query = Query::new(language, "(for_expression) @node").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .flat_map(|matched| {
            matched
                .captures
                .iter()
                .map(|cap| ForResult { node: cap.node })
        })
        .collect()
}

pub(crate) struct WhileResult<'a> {
    pub(crate) node: Node<'a>,
    pub(crate) condtion: Node<'a>,
    pub(crate) body: Node<'a>,
}

pub(crate) fn get_while_loops<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<WhileResult<'a>> {
    let query = Query::new(
        language,
        "(while_expression
             condition: (_) @condition
             body: (_) @body
         ) @node",
    )
    .unwrap();

    let node_idx = query.capture_index_for_name("node").unwrap();
    let condition_idx = query.capture_index_for_name("condition").unwrap();
    let body_idx = query.capture_index_for_name("body").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| WhileResult {
            node: matched
                .nodes_for_capture_index(node_idx)
                .next()
                .expect("one and only one while node"),
            condtion: matched
                .nodes_for_capture_index(condition_idx)
                .next()
                .expect("one and only one while condition"),
            body: matched
                .nodes_for_capture_index(body_idx)
                .next()
                .expect("one and only one while body"),
        })
        .collect()
}
