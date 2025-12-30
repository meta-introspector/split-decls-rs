// Generated macro for BidiMirroring (struct)
macro_rules! Depcrate_bidi_mirroring_glyphBidiMirroring {
() => {
// Module: crate::bidi_mirroring_glyph
// Provides: {"BidiMirroring"}
// Dependencies: {}
# [doc = " Represents a single row in the `BidiMirroring.txt` file."] # [doc = ""] # [doc = " The field names were taken from the header of BidiMirroring.txt."] # [derive (Clone , Debug , Default , Eq , PartialEq)] pub struct BidiMirroring { # [doc = " The codepoint corresponding to this row."] pub codepoint : Codepoint , # [doc = " The codepoint that has typically has a glyph that is the mirror image"] # [doc = " of `codepoint`."] pub bidi_mirroring_glyph : Codepoint , }
};
}
