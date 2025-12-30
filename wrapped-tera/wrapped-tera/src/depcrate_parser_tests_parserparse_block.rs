// Generated macro for parse_block (function)
macro_rules! Depcrate_parser_tests_parserparse_block {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_block"}
// Dependencies: {}
# [test] fn parse_block () { let ast = parse ("{% block hello %}{{super()}} hey{%- endblock hello %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Block (start_ws , Block { name : "hello" . to_string () , body : vec ! [Node :: Super , Node :: Text (" hey" . to_string ())] , } , end_ws ,)) ; }
};
}
