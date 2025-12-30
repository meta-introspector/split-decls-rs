// Generated macro for parse_break (function)
macro_rules! Depcrate_parser_tests_parserparse_break {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_break"}
// Dependencies: {}
# [test] fn parse_break () { let ast = parse ("{% for item in items %}{% break -%}{% endfor %}") . unwrap () ; let for_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Forloop (for_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Ident ("items" . to_string ())) , body : vec ! [Node :: Break (WS { left : false , right : true }) ,] , empty_body : None , } , for_ws ,)) ; }
};
}
