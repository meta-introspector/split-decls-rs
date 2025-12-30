// Generated macro for index_impl (macro)
macro_rules! Depcrate_tyindex_impl {
() => {
// Module: crate::ty
// Provides: {"index_impl"}
// Dependencies: {}
macro_rules ! index_impl { ($ name : ident) => { impl crate :: IndexedVal for $ name { fn to_val (index : usize) -> Self { $ name (index) } fn to_index (& self) -> usize { self . 0 } } } ; }
};
}
