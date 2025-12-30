// Generated macro for parse_continue (function)
macro_rules! Depcrate_parser_tests_parserparse_continue {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_continue"}
// Dependencies: {}
# [test] fn parse_continue () { let ast = parse ("{% for item in items %}{% continue -%}{% endfor %}") . unwrap () ; let for_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Forloop (for_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: new (ExprVal :: Ident ("items" . to_string ())) , body : vec ! [Node :: Continue (WS { left : false , right : true }) ,] , empty_body : None , } , for_ws ,)) ; }
};
}
