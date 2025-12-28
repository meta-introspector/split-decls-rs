macro_rules! deps {
    () => {
        MemoIngredientMap!();
        Id!();
        MemoIngredientIndices!();
        MemoIngredientIndex!();
        IngredientIndex!();
        Zalsa!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl MemoIngredientMap for MemoIngredientIndices { # [inline (always)] fn get_zalsa_id (& self , zalsa : & Zalsa , id : Id) -> MemoIngredientIndex { self . get (zalsa . ingredient_index (id)) } # [inline (always)] fn get (& self , index : IngredientIndex) -> MemoIngredientIndex { self . indices [index . as_u32 () as usize] } }
    };
}

impl_234!();