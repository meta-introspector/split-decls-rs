// Generated macro for array_like_impl (macro)
macro_rules! Depcrate_cnfarray_like_impl {
() => {
// Module: crate::cnf
// Provides: {"array_like_impl"}
// Dependencies: {}
macro_rules ! array_like_impl { ($ count : expr , $ ($ call : tt) *) => { impl < Item > UniformTuple < Item > for ($ (ignore_first ! ($ call , Item)) ,*) { fn tuple_len () -> usize { $ count } fn tuple_from_iter (mut items : impl Iterator < Item = Item >) -> Self { ($ (items . next () . unwrap () . into $ call) ,*) } } } }
};
}
