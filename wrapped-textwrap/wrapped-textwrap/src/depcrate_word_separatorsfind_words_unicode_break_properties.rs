// Generated macro for find_words_unicode_break_properties (function)
macro_rules! Depcrate_word_separatorsfind_words_unicode_break_properties {
() => {
// Module: crate::word_separators
// Provides: {"find_words_unicode_break_properties"}
// Dependencies: {}
# [doc = " Find words in line. ANSI escape sequences are ignored in `line`."] # [cfg (feature = "unicode-linebreak")] fn find_words_unicode_break_properties < 'a > (line : & 'a str ,) -> Box < dyn Iterator < Item = Word < 'a > > + 'a > { let mut last_stripped_idx = 0 ; let mut char_indices = line . char_indices () ; let mut idx_map = std :: iter :: from_fn (move | | match char_indices . next () { Some ((orig_idx , ch)) => { let stripped_idx = last_stripped_idx ; if ! skip_ansi_escape_sequence (ch , & mut char_indices . by_ref () . map (| (_ , ch) | ch)) { last_stripped_idx += ch . len_utf8 () ; } Some ((orig_idx , stripped_idx)) } None => None , }) ; let stripped = strip_ansi_escape_sequences (line) ; let mut opportunities = unicode_linebreak :: linebreaks (& stripped) . filter (| (idx , _) | { # [allow (clippy :: match_like_matches_macro)] match & stripped [.. * idx] . chars () . next_back () { Some ('-') => false , Some (SHY) => false , _ => true , } }) . collect :: < Vec < _ > > () . into_iter () ; opportunities . next_back () ; let mut start = 0 ; Box :: new (std :: iter :: from_fn (move | | { for (idx , _) in opportunities . by_ref () { if let Some ((orig_idx , _)) = idx_map . find (| & (_ , stripped_idx) | stripped_idx == idx) { let word = Word :: from (& line [start .. orig_idx]) ; start = orig_idx ; return Some (word) ; } } if start < line . len () { let word = Word :: from (& line [start ..]) ; start = line . len () ; return Some (word) ; } None })) }
};
}
