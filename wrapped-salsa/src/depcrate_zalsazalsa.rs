// Generated macro for Zalsa (struct)
macro_rules! Depcrate_zalsaZalsa {
() => {
// Module: crate::zalsa
// Provides: {"Zalsa"}
// Dependencies: {}
# [doc = " The \"plumbing interface\" to the Salsa database. Stores all the ingredients and other data."] # [doc = ""] # [doc = " **NOT SEMVER STABLE.**"] pub struct Zalsa { views_of : Views , # [cfg (not (feature = "inventory"))] nonce : crate :: nonce :: Nonce < StorageNonce > , # [doc = " Map from the [`IngredientIndex::as_usize`][] of a salsa struct to a list of"] # [doc = " [ingredient-indices](`IngredientIndex`) for tracked functions that have this salsa struct"] # [doc = " as input."] memo_ingredient_indices : Vec < Vec < IngredientIndex > > , # [doc = " Map from the type-id of an `impl Jar` to the index of its first ingredient."] jar_map : HashMap < TypeId , IngredientIndex , BuildHasherDefault < TypeIdHasher > > , # [doc = " A map from the `IngredientIndex` to the `TypeId` of its ID struct."] # [doc = ""] # [doc = " Notably this is not the reverse mapping of `jar_map`."] ingredient_to_id_struct_type_id_map : FxHashMap < IngredientIndex , TypeId > , # [doc = " Vector of ingredients."] ingredients_vec : Vec < Box < dyn Ingredient > > , # [doc = " Indices of ingredients that require reset when a new revision starts."] ingredients_requiring_reset : boxcar :: Vec < IngredientIndex > , # [doc = " The runtime for this particular salsa database handle."] # [doc = " Each handle gets its own runtime, but the runtimes have shared state between them."] runtime : Runtime , event_callback : Option < Box < dyn Fn (crate :: Event) + Send + Sync > > , }
};
}
