// Generated macro for to_ranges (function)
macro_rules! Depcrate_utilto_ranges {
() => {
// Module: crate::util
// Provides: {"to_ranges"}
// Dependencies: {}
# [doc = " Convert an iterator of codepoints into a vec of sorted ranges."] pub fn to_ranges < I : IntoIterator < Item = u32 > > (it : I) -> Vec < (u32 , u32) > { let mut codepoints : Vec < u32 > = it . into_iter () . collect () ; codepoints . sort () ; codepoints . dedup () ; let mut ranges = vec ! [] ; for cp in codepoints { range_add (& mut ranges , cp) ; } ranges }
};
}
