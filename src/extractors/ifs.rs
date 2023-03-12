use tree_sitter::{Language, Node, Query, QueryCursor};

pub(crate) struct IfResult<'a> {
    // pub(crate) node: Node<'a>,
    pub(crate) condtion: Node<'a>,
    // pub(crate) consequence: Node<'a>,
    // pub(crate) alternative: Option<Node<'a>>,
}

pub(crate) fn get_ifs<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<IfResult<'a>> {
    let query = Query::new(
        language,
        "(if_expression
          condition: (_) @condition
          consequence: (block) @consequence
          alternative: (_)? @alternative
         ) @node",
    )
    .unwrap();

    // let node_idx = query.capture_index_for_name("node").unwrap();
    let condition_idx = query.capture_index_for_name("condition").unwrap();
    // let consequence_idx = query.capture_index_for_name("consequence").unwrap();
    // let alternative_idx = query.capture_index_for_name("alternative").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| IfResult {
            // node: matched
            //     .nodes_for_capture_index(node_idx)
            //     .next()
            //     .expect("one and only one if node"),
            condtion: matched
                .nodes_for_capture_index(condition_idx)
                .next()
                .expect("one and only one if condition"),
            // consequence: matched
            //     .nodes_for_capture_index(consequence_idx)
            //     .next()
            //     .expect("one and only one if consequence"),

            // alternative: matched.nodes_for_capture_index(alternative_idx).next(),
        })
        .collect()
}

pub(crate) fn count_condition_variables(
    node: &Node,
    language: Language,
    source_code: &str,
) -> usize {
    let query = Query::new(
        language,
        "[
        (binary_expression operator: \"&&\")
        (binary_expression operator: \"||\")
        ] @binary_expressions",
    )
    .unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| matched.captures.len())
        .sum::<usize>()
        + 1
}
