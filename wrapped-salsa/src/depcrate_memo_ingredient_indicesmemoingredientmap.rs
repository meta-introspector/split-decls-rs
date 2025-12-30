// Generated macro for MemoIngredientMap (trait)
macro_rules! Depcrate_memo_ingredient_indicesMemoIngredientMap {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"MemoIngredientMap"}
// Dependencies: {}
pub trait MemoIngredientMap : Send + Sync { fn get_zalsa_id (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex ; fn get (& self , index : IngredientIndex) -> MemoIngredientIndex ; }
};
}
