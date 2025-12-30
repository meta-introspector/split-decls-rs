// Generated macro for to_range_values (function)
macro_rules! Depcrate_utilto_range_values {
() => {
// Module: crate::util
// Provides: {"to_range_values"}
// Dependencies: {}
# [doc = " Convert an iterator of codepoint-value associations into a vec of sorted"] # [doc = " ranges."] # [doc = ""] # [doc = " This panics if the same codepoint is present multiple times."] pub fn to_range_values < I , V > (it : I) -> Vec < (u32 , u32 , V) > where I : IntoIterator < Item = (u32 , V) > , V : Ord , { let mut codepoints : Vec < (u32 , V) > = it . into_iter () . collect () ; codepoints . sort () ; codepoints . dedup () ; let mut ranges = vec ! [] ; for (cp , value) in codepoints { range_value_add (& mut ranges , cp , value) ; } ranges }
};
}
