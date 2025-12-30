// Generated macro for Forloop (struct)
macro_rules! Depcrate_parser_astForloop {
() => {
// Module: crate::parser::ast
// Provides: {"Forloop"}
// Dependencies: {}
# [doc = " A forloop: can be over values or key/values"] # [derive (Clone , Debug , PartialEq)] pub struct Forloop { # [doc = " Name of the key in the loop (only when iterating on map-like objects)"] pub key : Option < String > , # [doc = " Name of the local variable for the value in the loop"] pub value : String , # [doc = " Expression being iterated on"] pub container : Expr , # [doc = " What's in the forloop itself"] pub body : Vec < Node > , # [doc = " The body to execute in case of an empty object"] pub empty_body : Option < Vec < Node > > , }
};
}
