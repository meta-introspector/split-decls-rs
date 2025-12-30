// Generated macro for impl_376 (impl)
macro_rules! Depcrate_memo_ingredient_indicesimpl_376 {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"impl_376"}
// Dependencies: {}
impl MemoIngredientMap for MemoIngredientIndices { # [inline (always)] fn get_zalsa_id (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex { self . get (zalsa . ingredient_index (id)) } # [inline (always)] fn get (& self , index : IngredientIndex) -> MemoIngredientIndex { self . indices [index . as_u32 () as usize] } }
};
}
