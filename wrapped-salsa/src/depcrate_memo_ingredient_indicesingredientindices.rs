// Generated macro for IngredientIndices (struct)
macro_rules! Depcrate_memo_ingredient_indicesIngredientIndices {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"IngredientIndices"}
// Dependencies: {}
# [doc = " An ingredient has an [ingredient index][IngredientIndex]. However, Salsa also supports"] # [doc = " enums of salsa structs (and other salsa enums), and those don't have a constant ingredient index,"] # [doc = " because they are not ingredients by themselves but rather composed of them. However, an enum can"] # [doc = " be viewed as a *set* of [`IngredientIndex`], where each instance of the enum can belong"] # [doc = " to one, potentially different, index. This is what this type represents: a set of"] # [doc = " `IngredientIndex`."] # [derive (Clone)] pub struct IngredientIndices { indices : Box < [IngredientIndex] > , }
};
}
