// Generated macro for parse_key_value_forloop (function)
macro_rules! Depcrate_parser_tests_parserparse_key_value_forloop {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_key_value_forloop"}
// Dependencies: {}
# [test] fn parse_key_value_forloop () { let ast = parse ("{% for key, item in get_map() %}A{%- endfor %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Forloop (start_ws , Forloop { key : Some ("key" . to_string ()) , value : "item" . to_string () , container : Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "get_map" . to_string () , args : HashMap :: new () , } ,)) , body : vec ! [Node :: Text ("A" . to_string ())] , empty_body : None , } , end_ws ,)) ; }
};
}
