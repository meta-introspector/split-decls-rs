// Generated macro for parse_array (function)
macro_rules! Depcrate_parserparse_array {
() => {
// Module: crate::parser
// Provides: {"parse_array"}
// Dependencies: {}
fn parse_array (pair : Pair < Rule >) -> TeraResult < ExprVal > { let mut vals = vec ! [] ; for p in pair . into_inner () { match p . as_rule () { Rule :: logic_val => { vals . push (parse_logic_val (p) ?) ; } _ => unreachable ! ("Got {:?} in parse_array" , p . as_rule ()) , } } Ok (ExprVal :: Array (vals)) }
};
}
