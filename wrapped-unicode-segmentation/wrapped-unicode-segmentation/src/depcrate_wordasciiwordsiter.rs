// Generated macro for AsciiWordsIter (type)
macro_rules! Depcrate_wordAsciiWordsIter {
() => {
// Module: crate::word
// Provides: {"AsciiWordsIter"}
// Dependencies: {}
type AsciiWordsIter < 'a > = Filter < core :: iter :: Map < AsciiWordBoundIter < 'a > , fn ((usize , & 'a str)) -> & 'a str > , fn (& & 'a str) -> bool , > ;
};
}
