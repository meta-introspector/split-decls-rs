macro_rules! MemoIngredientIndex {
    () => {
        # [doc = " A special secondary index *just* for ingredients that attach"] # [doc = " \"memos\" to salsa structs (currently: just tracked functions)."] # [derive (Copy , Clone , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] pub struct MemoIngredientIndex (u32) ;
    };
}

MemoIngredientIndex!()