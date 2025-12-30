// Generated macro for parse_exemplar_char_string (function)
macro_rules! Depcrate_charactersparse_exemplar_char_string {
() => {
// Module: crate::characters
// Provides: {"parse_exemplar_char_string"}
// Dependencies: {}
# [doc = " Parse the input CLDR JSON string representing exemplar character data and return a"] # [doc = " set of strings representing each code point or string represented by the CLDR JSON"] # [doc = " serialized form."] fn parse_exemplar_char_string (s : & str) -> HashSet < String > { debug_assert ! (s . starts_with ('[')) ; debug_assert ! (s . ends_with (']')) ; let mut transformed_input = s . split_at (1) . 1 . split_at (s . len () - 2) . 0 . to_string () ; if transformed_input . is_empty () { return HashSet :: new () ; } let mut dedup_chars = HashSet :: < String > :: new () ; preprocess_char_literal_notation (& mut dedup_chars , & mut transformed_input) ; transformed_input . split (is_exemplar_string_split_char) . filter (| t | ! t . is_empty ()) . for_each (| token | { let mut string_and_chars = token . split ('}') ; if let Some (maybe_char_string) = string_and_chars . next () { if ! maybe_char_string . is_empty () { if token . contains ('}') { let unescaped_char_string = unescape_exemplar_chars (maybe_char_string) ; dedup_chars . insert (unescaped_char_string) ; } else { let unescaped_char_block = unescape_exemplar_chars (maybe_char_string) ; insert_chars_from_string (& mut dedup_chars , & unescaped_char_block) ; } } for char_block in string_and_chars . filter (| t | ! t . is_empty ()) { let unescaped_char_block = unescape_exemplar_chars (char_block) ; insert_chars_from_string (& mut dedup_chars , & unescaped_char_block) ; } } }) ; dedup_chars }
};
}
