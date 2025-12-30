// Generated macro for impl_372 (impl)
macro_rules! Depcrate_memo_ingredient_indicesimpl_372 {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"impl_372"}
// Dependencies: {}
impl IngredientIndices { # [inline] pub fn empty () -> Self { Self { indices : Box :: default () , } } pub fn merge (iter : impl IntoIterator < Item = Self >) -> Self { let mut indices = Vec :: new () ; for index in iter { indices . extend (index . indices) ; } indices . sort_unstable () ; indices . dedup () ; Self { indices : indices . into_boxed_slice () , } } pub fn iter (& self) -> impl Iterator < Item = IngredientIndex > + '_ { self . indices . iter () . copied () } }
};
}
