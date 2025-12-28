macro_rules! deps {
    () => {
        Id!();
        MemoIngredientIndex!();
        Zalsa!();
        IngredientIndex!();
    };
}

macro_rules! MemoIngredientMap {
    () => {
        deps!();
        pub trait MemoIngredientMap : Send + Sync { fn get_zalsa_id (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex ; fn get (& self , index : IngredientIndex) -> MemoIngredientIndex ; }
    };
}

MemoIngredientMap!();