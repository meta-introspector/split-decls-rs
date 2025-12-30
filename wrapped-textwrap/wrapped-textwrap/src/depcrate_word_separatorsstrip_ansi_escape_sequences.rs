// Generated macro for strip_ansi_escape_sequences (function)
macro_rules! Depcrate_word_separatorsstrip_ansi_escape_sequences {
() => {
// Module: crate::word_separators
// Provides: {"strip_ansi_escape_sequences"}
// Dependencies: {}
# [cfg (feature = "unicode-linebreak")] fn strip_ansi_escape_sequences (text : & str) -> String { let mut result = String :: with_capacity (text . len ()) ; let mut chars = text . chars () ; while let Some (ch) = chars . next () { if skip_ansi_escape_sequence (ch , & mut chars) { continue ; } result . push (ch) ; } result }
};
}
