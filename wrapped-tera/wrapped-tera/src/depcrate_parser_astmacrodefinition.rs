// Generated macro for MacroDefinition (struct)
macro_rules! Depcrate_parser_astMacroDefinition {
() => {
// Module: crate::parser::ast
// Provides: {"MacroDefinition"}
// Dependencies: {}
# [doc = " A Macro definition"] # [derive (Clone , Debug , PartialEq)] pub struct MacroDefinition { # [doc = " The macro name"] pub name : String , # [doc = " The args for that macro: name -> optional default value"] pub args : HashMap < String , Option < Expr > > , # [doc = " The macro content"] pub body : Vec < Node > , }
};
}
