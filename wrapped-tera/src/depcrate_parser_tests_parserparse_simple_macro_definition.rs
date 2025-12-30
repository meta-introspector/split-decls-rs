// Generated macro for parse_simple_macro_definition (function)
macro_rules! Depcrate_parser_tests_parserparse_simple_macro_definition {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_simple_macro_definition"}
// Dependencies: {}
# [test] fn parse_simple_macro_definition () { let ast = parse ("{% macro hello(a=1, b='hello', c) %}A: {{a}}{% endmacro %}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("a" . to_string () , Some (Expr :: new (ExprVal :: Int (1)))) ; args . insert ("b" . to_string () , Some (Expr :: new (ExprVal :: String ("hello" . to_string ())))) ; args . insert ("c" . to_string () , None) ; assert_eq ! (ast [0] , Node :: MacroDefinition (WS :: default () , MacroDefinition { name : "hello" . to_string () , args , body : vec ! [Node :: Text ("A: " . to_string ()) , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Ident ("a" . to_string ()))) ,] , } , WS :: default () ,)) ; }
};
}
