// Generated macro for UnicodeWordsIter (type)
macro_rules! Depcrate_wordUnicodeWordsIter {
() => {
// Module: crate::word
// Provides: {"UnicodeWordsIter"}
// Dependencies: {}
type UnicodeWordsIter < 'a > = Filter < UWordBounds < 'a > , fn (& & 'a str) -> bool > ;
};
}
