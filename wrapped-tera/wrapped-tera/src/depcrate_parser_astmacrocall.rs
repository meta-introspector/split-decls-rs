// Generated macro for MacroCall (struct)
macro_rules! Depcrate_parser_astMacroCall {
() => {
// Module: crate::parser::ast
// Provides: {"MacroCall"}
// Dependencies: {}
# [doc = " A call to a namespaced macro `macros::my_macro()`"] # [derive (Clone , Debug , PartialEq)] pub struct MacroCall { # [doc = " The namespace we're looking for that macro in"] pub namespace : String , # [doc = " The macro name"] pub name : String , # [doc = " The args for that macro: name -> value"] pub args : HashMap < String , Expr > , }
};
}
