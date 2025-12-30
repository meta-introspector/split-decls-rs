// Generated macro for split_words (function)
macro_rules! Depcrate_word_splitterssplit_words {
() => {
// Module: crate::word_splitters
// Provides: {"split_words"}
// Dependencies: {}
# [doc = " Split words into smaller words according to the split points given"] # [doc = " by `word_splitter`."] # [doc = ""] # [doc = " Note that we split all words, regardless of their length. This is"] # [doc = " to more cleanly separate the business of splitting (including"] # [doc = " automatic hyphenation) from the business of word wrapping."] pub fn split_words < 'a , I > (words : I , word_splitter : & 'a WordSplitter ,) -> impl Iterator < Item = Word < 'a > > where I : IntoIterator < Item = Word < 'a > > , { words . into_iter () . flat_map (move | word | { let mut prev = 0 ; let mut split_points = word_splitter . split_points (& word) . into_iter () ; std :: iter :: from_fn (move | | { if let Some (idx) = split_points . next () { let need_hyphen = ! word [.. idx] . ends_with ('-') ; let w = Word { word : & word . word [prev .. idx] , width : display_width (& word [prev .. idx]) , whitespace : "" , penalty : if need_hyphen { "-" } else { "" } , } ; prev = idx ; return Some (w) ; } if prev < word . word . len () || prev == 0 { let w = Word { word : & word . word [prev ..] , width : display_width (& word [prev ..]) , whitespace : word . whitespace , penalty : word . penalty , } ; prev = word . word . len () + 1 ; return Some (w) ; } None }) }) }
};
}
