// Generated macro for is_punctuation (function)
macro_rules! Depcrate_stringis_punctuation {
() => {
// Module: crate::string
// Provides: {"is_punctuation"}
// Dependencies: {}
fn is_punctuation (grapheme : & str) -> bool { grapheme . chars () . all (| c | c . general_category () == GeneralCategory :: OtherPunctuation) }
};
}
