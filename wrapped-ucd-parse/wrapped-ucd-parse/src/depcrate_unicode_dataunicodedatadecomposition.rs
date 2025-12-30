// Generated macro for UnicodeDataDecomposition (struct)
macro_rules! Depcrate_unicode_dataUnicodeDataDecomposition {
() => {
// Module: crate::unicode_data
// Provides: {"UnicodeDataDecomposition"}
// Dependencies: {}
# [doc = " Represents a decomposition mapping of a single row in the"] # [doc = " `UnicodeData.txt` file."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct UnicodeDataDecomposition { # [doc = " The formatting tag associated with this mapping, if present."] pub tag : Option < UnicodeDataDecompositionTag > , # [doc = " The number of codepoints in this mapping."] pub len : usize , # [doc = " The codepoints in the mapping. Entries beyond `len` in the mapping"] # [doc = " are always U+0000. If no mapping was present, then this always contains"] # [doc = " a single codepoint corresponding to this row's character."] pub mapping : [Codepoint ; 18] , }
};
}
