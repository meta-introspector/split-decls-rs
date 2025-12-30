// Generated macro for IdentityHash (struct)
macro_rules! Depcrate_tracked_structIdentityHash {
() => {
// Module: crate::tracked_struct
// Provides: {"IdentityHash"}
// Dependencies: {}
# [doc = " Stores the data that (almost) uniquely identifies a tracked struct."] # [doc = ""] # [doc = " This includes the ingredient index of that struct type plus the hash of its untracked"] # [doc = " fields. This is mapped to a disambiguator -- a value that starts as 0 but increments"] # [doc = " each round, allowing for multiple tracked structs with the same hash and `IngredientIndex`"] # [doc = " created within the query to each have a unique ID."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Copy , Clone)] pub struct IdentityHash { # [doc = " Index of the tracked struct ingredient."] ingredient_index : IngredientIndex , # [doc = " Hash of the id fields."] hash : u64 , }
};
}
