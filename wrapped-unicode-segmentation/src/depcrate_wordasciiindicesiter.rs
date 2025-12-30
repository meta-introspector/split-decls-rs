// Generated macro for AsciiIndicesIter (type)
macro_rules! Depcrate_wordAsciiIndicesIter {
() => {
// Module: crate::word
// Provides: {"AsciiIndicesIter"}
// Dependencies: {}
type AsciiIndicesIter < 'a > = Filter < AsciiWordBoundIter < 'a > , fn (& (usize , & 'a str)) -> bool > ;
};
}
