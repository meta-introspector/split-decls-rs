// Generated macro for Test (struct)
macro_rules! Depcrate_parser_astTest {
() => {
// Module: crate::parser::ast
// Provides: {"Test"}
// Dependencies: {}
# [doc = " A test node `if my_var is odd`"] # [derive (Clone , Debug , PartialEq)] pub struct Test { # [doc = " Which variable is evaluated"] pub ident : String , # [doc = " Is it using `not`?"] pub negated : bool , # [doc = " Name of the test"] pub name : String , # [doc = " Any optional arg given to the test"] pub args : Vec < Expr > , }
};
}
