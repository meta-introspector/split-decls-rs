macro_rules! deps {
    () => {
        MemoIngredientSingletonIndex!();
        IngredientIndex!();
        Zalsa!();
        MemoIngredientMap!();
        Id!();
        MemoIngredientIndex!();
    };
}

macro_rules! impl_236 {
    () => {
        deps!();
        impl MemoIngredientMap for MemoIngredientSingletonIndex { # [inline (always)] fn get_zalsa_id (& self , _ : & Zalsa , _ : Id) -> MemoIngredientIndex { self . 0 } # [inline (always)] fn get (& self , _ : IngredientIndex) -> MemoIngredientIndex { self . 0 } }
    };
}

impl_236!();