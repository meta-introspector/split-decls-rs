// Generated macro for parse_value_forloop_array_with_filter (function)
macro_rules! Depcrate_parser_tests_parserparse_value_forloop_array_with_filter {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_value_forloop_array_with_filter"}
// Dependencies: {}
# [test] fn parse_value_forloop_array_with_filter () { let ast = parse ("{% for item in [1,2,] | reverse %}A{%- endfor %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Forloop (start_ws , Forloop { key : None , value : "item" . to_string () , container : Expr :: with_filters (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Int (2)) ,]) , vec ! [FunctionCall { name : "reverse" . to_string () , args : HashMap :: new () } ,] ,) , body : vec ! [Node :: Text ("A" . to_string ())] , empty_body : None , } , end_ws ,)) ; }
};
}
