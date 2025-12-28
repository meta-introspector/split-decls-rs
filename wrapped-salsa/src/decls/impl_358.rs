macro_rules! deps {
    () => {
        Identity!();
        IngredientIndex!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl Identity { pub (crate) fn ingredient_index (& self) -> IngredientIndex { self . ingredient_index } }
    };
}

impl_358!();