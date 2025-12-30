// Generated macro for index_words_string (function)
macro_rules! Depcrate_tendril_benchindex_words_string {
() => {
// Module: crate::tendril::bench
// Provides: {"index_words_string"}
// Dependencies: {}
fn index_words_string (input : & String) -> HashMap < char , Vec < String > > { let mut index = HashMap :: new () ; for word in input . split (| c | c == ' ') { if word . len () == 0 { continue ; } let word = word . to_owned () ; match index . entry (word . chars () . next () . unwrap ()) { Entry :: Occupied (mut e) => { let x : & mut Vec < String > = e . get_mut () ; x . push (word) ; } Entry :: Vacant (e) => { e . insert (vec ! [word]) ; } } } index }
};
}
