macro_rules! deps {
    () => {
        IngredientIndex!();
        Zalsa!();
        IngredientIndices!();
        NewMemoIngredientIndices!();
        MemoIngredientSingletonIndex!();
    };
}

macro_rules! impl_237 {
    () => {
        deps!();
        impl NewMemoIngredientIndices for MemoIngredientSingletonIndex { # [inline] unsafe fn create (zalsa : & mut Zalsa , indices : IngredientIndices , ingredient : IngredientIndex , memo_type : MemoEntryType , intern_ingredient_memo_types : Option < & mut Arc < MemoTableTypes > > ,) -> Self { let & [struct_ingredient] = & * indices . indices else { unreachable ! ("Attempting to construct struct memo mapping from enum?") } ; let memo_ingredient_index = zalsa . next_memo_ingredient_index (struct_ingredient , ingredient) ; let memo_types = intern_ingredient_memo_types . unwrap_or_else (| | { let (struct_ingredient , _) = zalsa . lookup_ingredient_mut (struct_ingredient) ; struct_ingredient . memo_table_types_mut () }) ; Arc :: get_mut (memo_types) . expect ("memo tables are not shared until database initialization is complete") . set (memo_ingredient_index , memo_type) ; Self (memo_ingredient_index) } }
    };
}

impl_237!()