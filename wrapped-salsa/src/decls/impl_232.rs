macro_rules! deps {
    () => {
        MemoIngredientIndex!();
        NewMemoIngredientIndices!();
        IngredientIndices!();
        Zalsa!();
        IngredientIndex!();
        MemoIngredientIndices!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl NewMemoIngredientIndices for MemoIngredientIndices { # [doc = " # Safety"] # [doc = ""] # [doc = " The memo types must be correct."] unsafe fn create (zalsa : & mut Zalsa , struct_indices : IngredientIndices , ingredient : IngredientIndex , memo_type : MemoEntryType , _intern_ingredient_memo_types : Option < & mut Arc < MemoTableTypes > > ,) -> Self { debug_assert ! (_intern_ingredient_memo_types . is_none () , "intern ingredient can only have a singleton memo ingredient") ; let Some (& last) = struct_indices . indices . last () else { unreachable ! ("Attempting to construct struct memo mapping for non tracked function?") } ; let mut indices = Vec :: new () ; indices . resize ((last . as_u32 () as usize) + 1 , MemoIngredientIndex :: from_usize ((u32 :: MAX - 1) as usize) ,) ; for & struct_ingredient in & struct_indices . indices { let memo_ingredient_index = zalsa . next_memo_ingredient_index (struct_ingredient , ingredient) ; indices [struct_ingredient . as_u32 () as usize] = memo_ingredient_index ; let (struct_ingredient , _) = zalsa . lookup_ingredient_mut (struct_ingredient) ; let memo_types = Arc :: get_mut (struct_ingredient . memo_table_types_mut ()) . expect ("memo tables are not shared until database initialization is complete") ; memo_types . set (memo_ingredient_index , memo_type) ; } MemoIngredientIndices { indices : indices . into_boxed_slice () , } } }
    };
}

impl_232!();