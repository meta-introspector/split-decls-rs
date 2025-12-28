macro_rules! deps {
    () => {
        IngredientIndex!();
        IngredientIndices!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl From < IngredientIndex > for IngredientIndices { # [inline] fn from (value : IngredientIndex) -> Self { Self { indices : Box :: new ([value]) , } } }
    };
}

impl_229!()