// Generated macro for parse_if (function)
macro_rules! Depcrate_parser_tests_parserparse_if {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_if"}
// Dependencies: {}
# [test] fn parse_if () { let ast = parse ("{% if item or admin %}A {%- elif 1 > 2 %}B{% else -%} C{%- endif %}") . unwrap () ; let end_ws = WS { left : true , .. Default :: default () } ; let else_ws = WS { right : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: If (If { conditions : vec ! [(WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("item" . to_string ()))) , operator : LogicOperator :: Or , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("admin" . to_string ()))) , })) , vec ! [Node :: Text ("A " . to_string ())] ,) , (end_ws , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : LogicOperator :: Gt , rhs : Box :: new (Expr :: new (ExprVal :: Int (2))) , })) , vec ! [Node :: Text ("B" . to_string ())] ,) ,] , otherwise : Some ((else_ws , vec ! [Node :: Text (" C" . to_string ())])) , } , end_ws ,)) ; }
};
}
