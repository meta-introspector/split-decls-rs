// Generated macro for impl_177 (impl)
macro_rules! Depcrate_bidi_mirroring_glyphimpl_177 {
() => {
// Module: crate::bidi_mirroring_glyph
// Provides: {"impl_177"}
// Dependencies: {}
impl std :: str :: FromStr for BidiMirroring { type Err = Error ; fn from_str (line : & str) -> Result < BidiMirroring , Error > { let re_parts = regex ! (r"(?x)
                ^
                \s*(?P<codepoint>[A-F0-9]+)\s*;
                \s*(?P<substitute_codepoint>[A-F0-9]+)
                \s+
                \#(?:.+)
                $
                " ,) ; let caps = match re_parts . captures (line . trim ()) { Some (caps) => caps , None => return err ! ("invalid BidiMirroring line") , } ; Ok (BidiMirroring { codepoint : caps ["codepoint"] . parse () ? , bidi_mirroring_glyph : caps ["substitute_codepoint"] . parse () ? , }) } }
};
}
