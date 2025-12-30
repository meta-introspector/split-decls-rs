// Generated macro for hangul_name (function)
macro_rules! Depcrate_hangulhangul_name {
() => {
// Module: crate::hangul
// Provides: {"hangul_name"}
// Dependencies: {}
# [doc = " Return the character name of the given precomposed Hangul codepoint."] # [doc = ""] # [doc = " If the given codepoint does not correspond to a precomposed Hangul"] # [doc = " codepoint in the inclusive range `AC00..D7A3`, then this returns `None`."] # [doc = ""] # [doc = " This implements the algorithms described in Unicode 3.12 and Unicode 4.8."] # [doc = ""] # [doc = " The `table` given should be a map from codepoint to the corresponding"] # [doc = " Jamo short name for that codepoint. If you're using `ucd-generate`, then"] # [doc = " the table can be generated via the `jamo-short-name` sub-command."] pub fn hangul_name < 'a > (table : & 'a [(u32 , & 'a str)] , cp : u32 ,) -> Option < String > { let mut name = "HANGUL SYLLABLE " . to_string () ; let (lpart , vpart , tpart) = match hangul_full_canonical_decomposition (cp) { None => return None , Some (triple) => triple , } ; name . push_str (jamo_short_name (table , lpart)) ; name . push_str (jamo_short_name (table , vpart)) ; name . push_str (tpart . map_or ("" , | cp | jamo_short_name (table , cp))) ; Some (name) }
};
}
