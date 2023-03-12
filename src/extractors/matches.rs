use tree_sitter::{Language, Node, Query, QueryCursor};

use crate::utils::trace_node;

pub(crate) struct MatchResult<'a> {
    pub(crate) node: Node<'a>,
    // pub(crate) body: Node<'a>,
    pub(crate) arms: Vec<Node<'a>>,
}

pub(crate) fn get_matches<'a>(
    node: &'a Node,
    language: Language,
    source_code: &'a str,
) -> Vec<MatchResult<'a>> {
    let query = Query::new(
        language,
        "(match_expression
          body: (match_block
            (match_arm)* @arms
          ) @body
         ) @node",
    )
    .unwrap();

    let node_idx = query.capture_index_for_name("node").unwrap();
    // let body_idx = query.capture_index_for_name("body").unwrap();
    let arm_idx = query.capture_index_for_name("arms").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| MatchResult {
            node: matched
                .nodes_for_capture_index(node_idx)
                .next()
                .expect("one and only one match node"),
            // body: matched
            //     .nodes_for_capture_index(body_idx)
            //     .next()
            //     .expect("one and only one body per match_expression"),

            arms: matched.nodes_for_capture_index(arm_idx).collect(),
        })
        .collect()
}

pub(crate) fn count_match_arm_condition_variables(
    node: &Node,
    language: Language,
    source_code: &str,
) -> usize {
    let query = Query::new(
        language,
        // "(match_pattern
        //     [
        //     (or_pattern) @or_pattern
        //     (identifier) @other
        //     (tuple_pattern) @other
        //     (tuple_struct_pattern) @other
        //     ]
        //  )",
        "(match_pattern
            (or_pattern)? @or_pattern
         ) @pattern",
    )
    .unwrap();

    let or_idx = query.capture_index_for_name("or_pattern").unwrap();
    let pattern_idx = query.capture_index_for_name("pattern").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| {
            matched.nodes_for_capture_index(or_idx).for_each(|node| trace_node(&node, "or match_pattern", source_code));
            matched.nodes_for_capture_index(pattern_idx).for_each(|node| trace_node(&node, "any match_pattern", source_code));

            let count_or_patterns: usize = matched.nodes_for_capture_index(or_idx).count();
            let count_total_patterns: usize = matched.nodes_for_capture_index(pattern_idx).count();
            let count_other_patterns: usize = count_total_patterns - count_or_patterns;

            let count_or_pattern_variables: usize = matched
                .nodes_for_capture_index(or_idx)
                .map(|node| count_or_pattern_variables(&node, language, source_code))
                .sum();
            tracing::trace!("#total: {count_total_patterns}, #or: {count_or_patterns}, #or_vars: {count_or_pattern_variables}");

            count_other_patterns + count_or_pattern_variables
        })
        .sum()
}

fn count_or_pattern_variables(node: &Node, language: Language, source_code: &str) -> usize {
    let query = Query::new(language, "(identifier) @identifier").unwrap();

    QueryCursor::new()
        .matches(&query, *node, source_code.as_bytes())
        .map(|matched| matched.captures.len())
        .sum()
}
