macro_rules! deps {
    () => {
        Zalsa!();
        IngredientIndices!();
        IngredientIndex!();
    };
}

macro_rules! NewMemoIngredientIndices {
    () => {
        deps!();
        pub trait NewMemoIngredientIndices { # [doc = " # Safety"] # [doc = ""] # [doc = " The memo types must be correct."] unsafe fn create (zalsa : & mut Zalsa , struct_indices : IngredientIndices , ingredient : IngredientIndex , memo_type : MemoEntryType , intern_ingredient_memo_types : Option < & mut Arc < MemoTableTypes > > ,) -> Self ; }
    };
}

NewMemoIngredientIndices!();