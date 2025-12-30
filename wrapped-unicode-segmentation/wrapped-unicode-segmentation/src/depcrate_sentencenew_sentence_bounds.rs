// Generated macro for new_sentence_bounds (function)
macro_rules! Depcrate_sentencenew_sentence_bounds {
() => {
// Module: crate::sentence
// Provides: {"new_sentence_bounds"}
// Dependencies: {}
# [inline] pub fn new_sentence_bounds (source : & str) -> USentenceBounds < '_ > { USentenceBounds { iter : fwd :: new_sentence_breaks (source) , sentence_start : None , } }
};
}
