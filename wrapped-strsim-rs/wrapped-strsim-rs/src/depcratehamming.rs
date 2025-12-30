// Generated macro for hamming (function)
macro_rules! Depcratehamming {
() => {
// Module: crate
// Provides: {"hamming"}
// Dependencies: {}
# [doc = " Calculates the number of positions in the two strings where the characters"] # [doc = " differ. Returns an error if the strings have different lengths."] # [doc = ""] # [doc = " ```"] # [doc = " use strsim::{hamming, StrSimError::DifferentLengthArgs};"] # [doc = ""] # [doc = " assert_eq!(Ok(3), hamming(\"hamming\", \"hammers\"));"] # [doc = ""] # [doc = " assert_eq!(Err(DifferentLengthArgs), hamming(\"hamming\", \"ham\"));"] # [doc = " ```"] pub fn hamming (a : & str , b : & str) -> HammingResult { generic_hamming (a . chars () , b . chars ()) }
};
}
