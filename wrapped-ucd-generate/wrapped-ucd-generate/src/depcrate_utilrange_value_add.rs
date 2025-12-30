// Generated macro for range_value_add (function)
macro_rules! Depcrate_utilrange_value_add {
() => {
// Module: crate::util
// Provides: {"range_value_add"}
// Dependencies: {}
# [doc = " Push a codepoint associated with a value onto a vec of ranges. If the"] # [doc = " codepoint belongs to the most recently added range and its value"] # [doc = " corresponds to the range's value, then increase the range to include this"] # [doc = " codepoint. Otherwise, add a new range containingly only the codepoint and"] # [doc = " value given."] # [doc = ""] # [doc = " This panics if the given codepoint is already in the ranges or if a"] # [doc = " codepoint is given out of order."] pub fn range_value_add < V : Eq > (ranges : & mut Vec < (u32 , u32 , V) > , codepoint : u32 , value : V ,) { if let Some (& mut (_ , ref mut end , ref value2)) = ranges . last_mut () { assert ! (* end < codepoint) ; if codepoint == * end + 1 && & value == value2 { * end = codepoint ; return ; } } ranges . push ((codepoint , codepoint , value)) ; }
};
}
