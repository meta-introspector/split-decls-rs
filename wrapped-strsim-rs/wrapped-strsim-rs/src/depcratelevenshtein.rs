// Generated macro for levenshtein (function)
macro_rules! Depcratelevenshtein {
() => {
// Module: crate
// Provides: {"levenshtein"}
// Dependencies: {}
# [doc = " Calculates the minimum number of insertions, deletions, and substitutions"] # [doc = " required to change one string into the other."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::levenshtein;"] # [doc = ""] # [doc = " assert_eq!(3, levenshtein(\"kitten\", \"sitting\"));"] # [doc = " ```"] pub fn levenshtein (a : & str , b : & str) -> usize { generic_levenshtein (& StringWrapper (a) , & StringWrapper (b)) }
};
}
