// Generated macro for macro_55 (macro)
macro_rules! Depcrate_exprmacro_55 {
() => {
// Module: crate::expr
// Provides: {"macro_55"}
// Dependencies: {}
# [cfg (feature = "full")] ast_struct ! { # [doc = " An arm of a 'match'."] # [doc = ""] # [doc = " E.g. `0...10 => { println!(\"match!\") }` as in"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " match n {"] # [doc = "     0...10 => { println!(\"match!\") },"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] pub struct Arm { pub attrs : Vec < Attribute >, pub pats : Delimited < Pat , tokens :: Or >, pub if_token : Option < tokens :: If >, pub guard : Option < Box < Expr >>, pub rocket_token : tokens :: Rocket , pub body : Box < Expr >, pub comma : Option < tokens :: Comma >, } }
};
}
