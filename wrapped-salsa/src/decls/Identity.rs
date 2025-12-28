macro_rules! deps {
    () => {
        IngredientIndex!();
        Disambiguator!();
    };
}

macro_rules! Identity {
    () => {
        deps!();
        # [doc = " Defines the identity of a tracked struct."] # [doc = " This is the key to a hashmap that is (initially)"] # [doc = " stored in the [`ActiveQuery`](`crate::active_query::ActiveQuery`)"] # [doc = " struct and later moved to the [`Memo`](`crate::function::memo::Memo`)."] # [derive (Debug , PartialEq , Eq , PartialOrd , Ord , Copy , Clone)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] pub (crate) struct Identity { # [doc = " Index of the tracked struct ingredient."] ingredient_index : IngredientIndex , # [doc = " Hash of the id fields."] hash : u64 , # [doc = " The unique disambiguator assigned within the active query"] # [doc = " to distinguish distinct tracked structs with the same identity_hash."] disambiguator : Disambiguator , }
    };
}

Identity!();