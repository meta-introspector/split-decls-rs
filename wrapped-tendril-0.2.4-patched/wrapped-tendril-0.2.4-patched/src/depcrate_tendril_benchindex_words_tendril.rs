// Generated macro for index_words_tendril (function)
macro_rules! Depcrate_tendril_benchindex_words_tendril {
() => {
// Module: crate::tendril::bench
// Provides: {"index_words_tendril"}
// Dependencies: {}
fn index_words_tendril (input : & StrTendril) -> HashMap < char , Vec < StrTendril > > { let mut index = HashMap :: new () ; let mut t = input . clone () ; loop { match t . pop_front_char_run (| c | c != ' ') { None => return index , Some ((_ , false)) => () , Some ((word , true)) => match index . entry (word . chars () . next () . unwrap ()) { Entry :: Occupied (mut e) => { e . get_mut () . push (word) ; } Entry :: Vacant (e) => { e . insert (vec ! [word]) ; } } } } }
};
}
