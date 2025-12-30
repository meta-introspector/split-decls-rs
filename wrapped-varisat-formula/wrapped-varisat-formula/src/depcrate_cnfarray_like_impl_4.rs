// Generated macro for array_like_impl_4 (macro)
macro_rules! Depcrate_cnfarray_like_impl_4 {
() => {
// Module: crate::cnf
// Provides: {"array_like_impl_4"}
// Dependencies: {}
macro_rules ! array_like_impl_4 { ($ count : expr , $ ($ call : tt) *) => { array_like_impl ! ($ count * 4 + 2 , $ (() () () $ call) * () ()) ; array_like_impl ! ($ count * 4 + 3 , $ (() () () $ call) * () () ()) ; array_like_impl ! ($ count * 4 + 4 , $ (() () () $ call) * () () () ()) ; array_like_impl ! ($ count * 4 + 5 , $ (() () () $ call) * () () () () ()) ; } }
};
}
