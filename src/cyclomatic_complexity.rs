use std::{fs, path::Path};
use tree_sitter::{Language, Node, Parser};

use crate::extractors::{
    get_functions,
    ifs::{count_condition_variables, get_ifs},
    iters::get_iters,
    loops::{get_for_loops, get_while_loops},
    matches::{count_match_arm_condition_variables, get_matches},
};
use crate::utils::trace_node;

pub fn parse(path: impl AsRef<Path>, parser: &mut Parser, language: Language) {
    let source_code = fs::read_to_string(path.as_ref()).unwrap();
    let tree = parser.parse(&source_code, None).unwrap();

    let root_node = tree.root_node();
    let functions = get_functions(&root_node, language, &source_code);

    functions.iter().for_each(|(fn_name, fn_node)| {
        trace_node(fn_node, fn_name, &source_code);
        let cc = cyclomatic_complexity(fn_node, language, &source_code);
        tracing::info!("{fn_name} - {cc}");
    });
}

fn cyclomatic_complexity(node: &Node, language: Language, source_code: &str) -> usize {
    let mut decision_points = 1;

    decision_points += get_ifs(node, language, source_code)
        .iter()
        .map(|if_result| count_condition_variables(&if_result.condtion, language, source_code))
        .sum::<usize>();

    // FIXME(lsp): detect non-exhaustive enums
    // matches in rust are "usually" exhaustive,
    // therefore cyclomatic complexity is number of match arm variables - 1
    // since one arm is effectively an "else"
    // For non-exhaustive enums the complexity is just the number of match arms.
    let matches = get_matches(node, language, source_code);
    matches
        .iter()
        .for_each(|matched| trace_node(&matched.node, "match", source_code));

    decision_points += matches
        .iter()
        .map(|mr| {
            mr.arms
                .iter()
                .map(|arm| {
                    let n_conditions =
                        count_match_arm_condition_variables(arm, language, source_code);
                    trace_node(arm, &format!("match_arm({n_conditions})"), source_code);
                    n_conditions
                })
                .sum::<usize>()
                - 1
        })
        .sum::<usize>();

    decision_points += get_for_loops(node, language, source_code).len();

    decision_points += get_while_loops(node, language, source_code)
        .iter()
        .map(|while_result| {
            count_condition_variables(&while_result.condtion, language, source_code)
        })
        .sum::<usize>();

    decision_points += get_iters(node, language, source_code).len();

    decision_points
}
