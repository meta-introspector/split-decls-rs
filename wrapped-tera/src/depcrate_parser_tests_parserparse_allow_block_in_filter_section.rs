// Generated macro for parse_allow_block_in_filter_section (function)
macro_rules! Depcrate_parser_tests_parserparse_allow_block_in_filter_section {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_allow_block_in_filter_section"}
// Dependencies: {}
# [test] fn parse_allow_block_in_filter_section () { let ast = parse ("{% filter upper %}{% block content %}Hello{% endblock %}{% endfilter %}") . unwrap () ; assert_eq ! (ast [0] , Node :: FilterSection (WS :: default () , FilterSection { filter : FunctionCall { name : "upper" . to_owned () , args : HashMap :: default () } , body : vec ! [Node :: Block (WS :: default () , Block { name : "content" . to_owned () , body : vec ! [Node :: Text ("Hello" . to_owned ())] } , WS :: default () ,)] , } , WS :: default () ,)) ; }
};
}
