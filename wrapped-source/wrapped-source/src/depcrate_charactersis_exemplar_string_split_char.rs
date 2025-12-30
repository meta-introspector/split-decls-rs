// Generated macro for is_exemplar_string_split_char (function)
macro_rules! Depcrate_charactersis_exemplar_string_split_char {
() => {
// Module: crate::characters
// Provides: {"is_exemplar_string_split_char"}
// Dependencies: {}
# [doc = " Predicate fn that returns whether a character should be used in `.split()` to tokenize"] # [doc = " the exemplar characters JSON string."] fn is_exemplar_string_split_char (c : char) -> bool { c . is_whitespace () || c == '{' }
};
}
