// Generated macro for macro_52 (macro)
macro_rules! Depcrate_exprmacro_52 {
() => {
// Module: crate::expr
// Provides: {"macro_52"}
// Dependencies: {}
# [cfg (feature = "full")] ast_enum ! { # [doc = " How a macro was invoked."] # [cfg_attr (feature = "clone-impls" , derive (Copy))] pub enum MacStmtStyle { # [doc = " The macro statement had a trailing semicolon, e.g. `foo! { ... };`"] # [doc = " `foo!(...);`, `foo![...];`"] Semicolon (tokens :: Semi) , # [doc = " The macro statement had braces; e.g. foo! { ... }"] Braces , # [doc = " The macro statement had parentheses or brackets and no semicolon; e.g."] # [doc = " `foo!(...)`. All of these will end up being converted into macro"] # [doc = " expressions."] NoBraces , } }
};
}
