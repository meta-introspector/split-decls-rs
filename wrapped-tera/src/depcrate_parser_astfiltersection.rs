// Generated macro for FilterSection (struct)
macro_rules! Depcrate_parser_astFilterSection {
() => {
// Module: crate::parser::ast
// Provides: {"FilterSection"}
// Dependencies: {}
# [doc = " A filter section node `{{ filter name(param=\"value\") }} content {{ endfilter }}`"] # [derive (Clone , Debug , PartialEq)] pub struct FilterSection { # [doc = " The filter call itsel"] pub filter : FunctionCall , # [doc = " The filter body"] pub body : Vec < Node > , }
};
}
