// Generated macro for macro_53 (macro)
macro_rules! Depcrate_exprmacro_53 {
() => {
// Module: crate::expr
// Provides: {"macro_53"}
// Dependencies: {}
# [cfg (feature = "full")] ast_struct ! { # [doc = " Local represents a `let` statement, e.g., `let <pat>:<ty> = <expr>;`"] pub struct Local { pub let_token : tokens :: Let , pub colon_token : Option < tokens :: Colon >, pub eq_token : Option < tokens :: Eq >, pub semi_token : tokens :: Semi , pub pat : Box < Pat >, pub ty : Option < Box < Ty >>, # [doc = " Initializer expression to set the value, if any"] pub init : Option < Box < Expr >>, pub attrs : Vec < Attribute >, } }
};
}
