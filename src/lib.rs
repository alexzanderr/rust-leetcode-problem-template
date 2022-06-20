
use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn {{function_name}}({{input}}; {{input_type}}) -> {{function_result}} {

    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use pretty_assertions::assert_eq;


    #[rstest]
    #[case(input, expected_result)]
    #[case()]
    #[case()]
    fn it_works(
        #[case] {{input}}: ,
        #[case] expected_result: ,
    ) {
        let result = Solution::{{function_name}}({{input}});
        assert_eq!(result, expected_result);
    }
}
