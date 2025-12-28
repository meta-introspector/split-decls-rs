macro_rules! deps {
    () => {
        Id!();
        IngredientIndex!();
    };
}

macro_rules! DatabaseKeyIndex {
    () => {
        deps!();
        # [doc = " An integer that uniquely identifies a particular query instance within the"] # [doc = " database. Used to track input and output dependencies between queries. Fully"] # [doc = " ordered and equatable but those orderings are arbitrary, and meant to be used"] # [doc = " only for inserting into maps and the like."] # [derive (Copy , Clone , PartialEq , Eq , Hash)] pub struct DatabaseKeyIndex { key_index : Id , ingredient_index : IngredientIndex , }
    };
}

DatabaseKeyIndex!()