// Generated macro for remove_all_iter (function)
macro_rules! Depcrate_tedremove_all_iter {
() => {
// Module: crate::ted
// Provides: {"remove_all_iter"}
// Dependencies: {}
pub fn remove_all_iter (range : impl IntoIterator < Item = SyntaxElement >) { let mut it = range . into_iter () ; if let Some (mut first) = it . next () { match it . last () { Some (mut last) => { if first . index () > last . index () { mem :: swap (& mut first , & mut last) ; } remove_all (first ..= last) ; } None => remove (first) , } } }
};
}
