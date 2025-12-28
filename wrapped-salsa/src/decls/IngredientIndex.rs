macro_rules! deps {
    () => {
        Ingredient!();
    };
}

macro_rules! IngredientIndex {
    () => {
        deps!();
        # [doc = " An ingredient index identifies a particular [`Ingredient`] in the database."] # [doc = ""] # [doc = " The database contains a number of jars, and each jar contains a number of ingredients."] # [doc = " Each ingredient is given a unique index as the database is being created."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "persistence" , derive (serde :: Serialize , serde :: Deserialize))] # [cfg_attr (feature = "persistence" , serde (transparent))] pub struct IngredientIndex (u32) ;
    };
}

IngredientIndex!()