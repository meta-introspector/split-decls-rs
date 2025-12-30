// Generated macro for range_add (function)
macro_rules! Depcrate_utilrange_add {
() => {
// Module: crate::util
// Provides: {"range_add"}
// Dependencies: {}
# [doc = " Push a codepoint onto a vec of ranges. If the codepoint belongs to the"] # [doc = " most recently added range, then increase the range. Otherwise, add a new"] # [doc = " range containing only the codepoint given."] # [doc = ""] # [doc = " This panics if the given codepoint is already in the ranges or if a"] # [doc = " codepoint is given out of order."] pub fn range_add (ranges : & mut Vec < (u32 , u32) > , codepoint : u32) { if let Some (& mut (_ , ref mut end)) = ranges . last_mut () { assert ! (* end < codepoint) ; if codepoint == * end + 1 { * end = codepoint ; return ; } } ranges . push ((codepoint , codepoint)) ; }
};
}
