// Generated macro for UnicodeIndicesIter (type)
macro_rules! Depcrate_wordUnicodeIndicesIter {
() => {
// Module: crate::word
// Provides: {"UnicodeIndicesIter"}
// Dependencies: {}
type UnicodeIndicesIter < 'a > = Filter < UWordBoundIndices < 'a > , fn (& (usize , & 'a str)) -> bool > ;
};
}
