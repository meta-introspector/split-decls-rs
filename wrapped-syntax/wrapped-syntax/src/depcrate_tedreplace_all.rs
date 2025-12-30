// Generated macro for replace_all (function)
macro_rules! Depcrate_tedreplace_all {
() => {
// Module: crate::ted
// Provides: {"replace_all"}
// Dependencies: {}
pub fn replace_all (range : RangeInclusive < SyntaxElement > , new : Vec < SyntaxElement >) { let start = range . start () . index () ; let end = range . end () . index () ; let parent = range . start () . parent () . unwrap () ; parent . splice_children (start .. end + 1 , new) ; }
};
}
