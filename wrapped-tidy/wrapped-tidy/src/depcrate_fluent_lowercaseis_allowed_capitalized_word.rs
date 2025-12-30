// Generated macro for is_allowed_capitalized_word (function)
macro_rules! Depcrate_fluent_lowercaseis_allowed_capitalized_word {
() => {
// Module: crate::fluent_lowercase
// Provides: {"is_allowed_capitalized_word"}
// Dependencies: {}
fn is_allowed_capitalized_word (msg : & str) -> bool { ALLOWED_CAPITALIZED_WORDS . iter () . any (| word | { msg . strip_prefix (word) . map (| tail | tail . chars () . next () . map (| c | c == '-' || c . is_whitespace ()) . unwrap_or (true)) . unwrap_or_default () }) }
};
}
