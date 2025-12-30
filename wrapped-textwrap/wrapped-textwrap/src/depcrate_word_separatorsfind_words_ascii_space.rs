// Generated macro for find_words_ascii_space (function)
macro_rules! Depcrate_word_separatorsfind_words_ascii_space {
() => {
// Module: crate::word_separators
// Provides: {"find_words_ascii_space"}
// Dependencies: {}
fn find_words_ascii_space < 'a > (line : & 'a str) -> Box < dyn Iterator < Item = Word < 'a > > + 'a > { let mut start = 0 ; let mut in_whitespace = false ; let mut char_indices = line . char_indices () ; Box :: new (std :: iter :: from_fn (move | | { for (idx , ch) in char_indices . by_ref () { if in_whitespace && ch != ' ' { let word = Word :: from (& line [start .. idx]) ; start = idx ; in_whitespace = ch == ' ' ; return Some (word) ; } in_whitespace = ch == ' ' ; } if start < line . len () { let word = Word :: from (& line [start ..]) ; start = line . len () ; return Some (word) ; } None })) }
};
}
