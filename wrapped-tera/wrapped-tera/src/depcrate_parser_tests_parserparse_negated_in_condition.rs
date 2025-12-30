// Generated macro for parse_negated_in_condition (function)
macro_rules! Depcrate_parser_tests_parserparse_negated_in_condition {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_negated_in_condition"}
// Dependencies: {}
# [test] fn parse_negated_in_condition () { let ast = parse ("{{ b not in c }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: In (In { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("b" . to_string ()))) , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("c" . to_string ()))) , negated : true , })))) ; }
};
}
