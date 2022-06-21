
#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_macros,
    unused_assignments,
    unused_mut,
    non_snake_case,
    unused_must_use,
    non_upper_case_globals,
    non_camel_case_types,
)]


mod solution;


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;
    use solution::Solution;

    #[rstest]
    #[case(input, expected_result)]
    #[case()]
    #[case()]
    fn it_works(
        #[case] {{input}}: {{input_type}},
        #[case] expected_result: {{function_result_type}},
    ) {
        let result = Solution::{{function_name}}({{input}});
        assert_eq!(result, expected_result);
    }
}
