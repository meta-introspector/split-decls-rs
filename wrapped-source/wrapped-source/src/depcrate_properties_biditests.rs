// Generated macro for tests (module)
macro_rules! Depcrate_properties_biditests {
() => {
// Module: crate::properties::bidi
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use icu :: properties :: props :: { BidiMirroringGlyph , BidiPairedBracketType } ; # [test] fn test_bidi_data_provider () { let provider = SourceDataProvider :: new_testing () ; let bidi_data = icu :: properties :: CodePointMapData :: < BidiMirroringGlyph > :: try_new_unstable (& provider) . unwrap () ; let bidi_data = bidi_data . as_borrowed () ; let close_paren = bidi_data . get (')') ; assert_eq ! (close_paren . mirroring_glyph , Some ('(')) ; assert ! (close_paren . mirrored) ; let close_angle_bracket = bidi_data . get ('>') ; assert_eq ! (close_angle_bracket . mirroring_glyph , Some ('<')) ; assert ! (close_angle_bracket . mirrored) ; let open_paren = bidi_data . get ('(') ; assert_eq ! (open_paren . paired_bracket_type , BidiPairedBracketType :: Open) ; let open_angle_bracket = bidi_data . get ('<') ; assert_eq ! (open_angle_bracket . paired_bracket_type , BidiPairedBracketType :: None) ; } }
};
}
