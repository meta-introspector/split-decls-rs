// Generated macro for MemoIngredientIndices (struct)
macro_rules! Depcrate_memo_ingredient_indicesMemoIngredientIndices {
() => {
// Module: crate::memo_ingredient_indices
// Provides: {"MemoIngredientIndices"}
// Dependencies: {}
# [doc = " This type is to [`MemoIngredientIndex`] what [`IngredientIndices`] is to [`IngredientIndex`]:"] # [doc = " since enums can contain different ingredient indices, they can also have different memo indices,"] # [doc = " so we need to keep track of them."] # [doc = ""] # [doc = " This acts a map from [`IngredientIndex`] to [`MemoIngredientIndex`] but implemented"] # [doc = " via a slice for fast lookups, trading memory for speed. With these changes, lookups are `O(1)`"] # [doc = " instead of `O(n)`."] # [doc = ""] # [doc = " A database tends to have few ingredients (i), less function ingredients and even less"] # [doc = " function ingredients targeting `#[derive(Supertype)]` enums (e)."] # [doc = " While this is bounded as `O(i * e)` memory usage, the average case is significantly smaller: a"] # [doc = " function ingredient targeting enums only stores a slice whose length corresponds to the largest"] # [doc = " ingredient index's _value_. For example, if we have the ingredient indices `[2, 6, 17]`, then we"] # [doc = " will allocate a slice whose length is `17 + 1`."] # [doc = ""] # [doc = " Assuming a heavy example scenario of 1000 ingredients (500 of which are function ingredients, 100"] # [doc = " of which are enum targeting functions) this would come out to a maximum possibly memory usage of"] # [doc = " 4bytes * 1000 * 100 ~= 0.38MB which is negligible."] pub struct MemoIngredientIndices { indices : Box < [MemoIngredientIndex] > , }
};
}
