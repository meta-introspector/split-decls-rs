// Generated macro for tests (module)
macro_rules! Depcrate_bidi_mirroring_glyphtests {
() => {
// Module: crate::bidi_mirroring_glyph
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: common :: Codepoint ; use super :: BidiMirroring ; fn codepoint (n : u32) -> Codepoint { Codepoint :: from_u32 (n) . unwrap () } # [test] fn parse () { let line = "0028; 0029 # LEFT PARENTHESIS\n" ; let data : BidiMirroring = line . parse () . unwrap () ; assert_eq ! (data , BidiMirroring { codepoint : codepoint (0x0028) , bidi_mirroring_glyph : codepoint (0x0029) , }) ; } # [test] fn parse_best_fit () { let line = "228A; 228B # [BEST FIT] SUBSET OF WITH NOT EQUAL TO\n" ; let data : BidiMirroring = line . parse () . unwrap () ; assert_eq ! (data , BidiMirroring { codepoint : codepoint (0x228A) , bidi_mirroring_glyph : codepoint (0x228B) , }) ; } }
};
}
