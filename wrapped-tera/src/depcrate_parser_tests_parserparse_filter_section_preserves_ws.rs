// Generated macro for parse_filter_section_preserves_ws (function)
macro_rules! Depcrate_parser_tests_parserparse_filter_section_preserves_ws {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_filter_section_preserves_ws"}
// Dependencies: {}
# [test] fn parse_filter_section_preserves_ws () { let ast = parse ("{% filter upper %}  {{a}}  B  {% endfilter %}") . unwrap () ; assert_eq ! (ast [0] , Node :: FilterSection (WS :: default () , FilterSection { filter : FunctionCall { name : "upper" . to_string () , args : HashMap :: new () } , body : vec ! [Node :: Text ("  " . to_string ()) , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Ident ("a" . to_string ()))) , Node :: Text ("  B  " . to_string ())] } , WS :: default () ,)) ; }
};
}
