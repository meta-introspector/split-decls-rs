// Generated macro for damerau_levenshtein (function)
macro_rules! Depcratedamerau_levenshtein {
() => {
// Module: crate
// Provides: {"damerau_levenshtein"}
// Dependencies: {}
# [doc = " Like optimal string alignment, but substrings can be edited an unlimited"] # [doc = " number of times, and the triangle inequality holds."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::damerau_levenshtein;"] # [doc = ""] # [doc = " assert_eq!(2, damerau_levenshtein(\"ab\", \"bca\"));"] # [doc = " ```"] pub fn damerau_levenshtein (a : & str , b : & str) -> usize { damerau_levenshtein_impl (a . chars () , a . chars () . count () , b . chars () , b . chars () . count ()) }
};
}
