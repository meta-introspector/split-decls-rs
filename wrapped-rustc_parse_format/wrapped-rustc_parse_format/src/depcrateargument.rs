// Generated macro for Argument (struct)
macro_rules! DepcrateArgument {
() => {
// Module: crate
// Provides: {"Argument"}
// Dependencies: {}
# [doc = " Representation of an argument specification."] # [derive (Clone , Debug , PartialEq)] pub struct Argument < 'input > { # [doc = " Where to find this argument"] pub position : Position < 'input > , # [doc = " The span of the position indicator. Includes any whitespace in implicit"] # [doc = " positions (`{  }`)."] pub position_span : Range < usize > , # [doc = " How to format the argument"] pub format : FormatSpec < 'input > , }
};
}
