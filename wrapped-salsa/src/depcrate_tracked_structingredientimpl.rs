// Generated macro for IngredientImpl (struct)
macro_rules! Depcrate_tracked_structIngredientImpl {
() => {
// Module: crate::tracked_struct
// Provides: {"IngredientImpl"}
// Dependencies: {}
# [doc = " Created for each tracked struct."] # [doc = ""] # [doc = " This ingredient only stores the \"id\" fields. It is a kind of \"dressed up\" interner;"] # [doc = " the active query + values of id fields are hashed to create the tracked"] # [doc = " struct id. The value fields are stored in [`crate::function::IngredientImpl`]"] # [doc = " instances keyed by the tracked struct id."] # [doc = ""] # [doc = " Unlike normal interned values, tracked struct indices can be deleted and reused aggressively"] # [doc = " without dependency edges on the creating query. When a tracked function is collected,"] # [doc = " any tracked structs it created can be deleted. Additionally, when a tracked function"] # [doc = " re-executes but does not create a tracked struct that was previously created, it can"] # [doc = " be deleted. No dependency edge is required as the lifetime of a tracked struct is tied"] # [doc = " directly to the query that created it."] pub struct IngredientImpl < C > where C : Configuration , { # [doc = " Our index in the database."] ingredient_index : IngredientIndex , # [doc = " Phantom data: we fetch `Value<C>` out from `Table`"] phantom : PhantomData < fn () -> Value < C > > , # [doc = " Store freed ids"] free_list : SegQueue < Id > , memo_table_types : Arc < MemoTableTypes > , }
};
}
