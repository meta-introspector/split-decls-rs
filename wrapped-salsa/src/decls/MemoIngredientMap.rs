macro_rules! deps {
    () => {
        MemoIngredientIndex!();
        IngredientIndex!();
        Zalsa!();
        Id!();
    };
}

macro_rules! MemoIngredientMap {
    () => {
        deps!();
        pub trait MemoIngredientMap : Send + Sync { fn get_zalsa_id (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex ; fn get (& self , index : IngredientIndex) -> MemoIngredientIndex ; }
    };
}

MemoIngredientMap!()