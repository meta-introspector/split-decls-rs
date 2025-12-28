macro_rules! deps {
    () => {
        MemoIngredientIndex!();
    };
}

macro_rules! MemoIngredientSingletonIndex {
    () => {
        deps!();
        # [derive (Debug)] pub struct MemoIngredientSingletonIndex (MemoIngredientIndex) ;
    };
}

MemoIngredientSingletonIndex!()