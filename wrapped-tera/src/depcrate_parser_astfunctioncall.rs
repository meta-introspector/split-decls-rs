// Generated macro for FunctionCall (struct)
macro_rules! Depcrate_parser_astFunctionCall {
() => {
// Module: crate::parser::ast
// Provides: {"FunctionCall"}
// Dependencies: {}
# [doc = " A function call, can be a filter or a global function"] # [derive (Clone , Debug , PartialEq)] pub struct FunctionCall { # [doc = " The name of the function"] pub name : String , # [doc = " The args of the function: key -> value"] pub args : HashMap < String , Expr > , }
};
}
