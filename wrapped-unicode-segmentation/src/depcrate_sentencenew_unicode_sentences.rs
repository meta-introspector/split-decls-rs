// Generated macro for new_unicode_sentences (function)
macro_rules! Depcrate_sentencenew_unicode_sentences {
() => {
// Module: crate::sentence
// Provides: {"new_unicode_sentences"}
// Dependencies: {}
# [inline] pub fn new_unicode_sentences (s : & str) -> UnicodeSentences < '_ > { use super :: UnicodeSegmentation ; use crate :: tables :: util :: is_alphanumeric ; fn has_alphanumeric (s : & & str) -> bool { s . chars () . any (is_alphanumeric) } let has_alphanumeric : fn (& & str) -> bool = has_alphanumeric ; UnicodeSentences { inner : s . split_sentence_bounds () . filter (has_alphanumeric) , } }
};
}
