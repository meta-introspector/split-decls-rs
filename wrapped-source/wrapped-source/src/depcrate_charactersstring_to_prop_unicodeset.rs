// Generated macro for string_to_prop_unicodeset (function)
macro_rules! Depcrate_charactersstring_to_prop_unicodeset {
() => {
// Module: crate::characters
// Provides: {"string_to_prop_unicodeset"}
// Dependencies: {}
fn string_to_prop_unicodeset (s : & str) -> ExemplarCharactersData < 'static > { ExemplarCharactersData (CodePointInversionListAndStringList :: from_iter (parse_exemplar_char_string (s) . iter () . map (Deref :: deref) . sorted () ,)) }
};
}
