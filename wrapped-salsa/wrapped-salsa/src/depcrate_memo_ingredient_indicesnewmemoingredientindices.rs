// Generated macro for NewMemoIngredientIndices (trait)
macro_rules! Depcrate_memo_ingredient_indicesNewMemoIngredientIndices {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"NewMemoIngredientIndices"}
// Dependencies: {}
pub trait NewMemoIngredientIndices { # [doc = " # Safety"] # [doc = ""] # [doc = " The memo types must be correct."] unsafe fn create (zalsa : & mut Zalsa , struct_indices : IngredientIndices , ingredient : IngredientIndex , memo_type : MemoEntryType , intern_ingredient_memo_types : Option < & mut Arc < MemoTableTypes > > ,) -> Self ; }
};
}
