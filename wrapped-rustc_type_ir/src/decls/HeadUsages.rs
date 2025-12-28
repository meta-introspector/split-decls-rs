macro_rules! HeadUsages {
    () => {
        # [doc = " The kinds of cycles a cycle head was involved in."] # [doc = ""] # [doc = " This is used to avoid rerunning a cycle if there's"] # [doc = " just a single usage kind and the final result matches"] # [doc = " its provisional result."] # [doc = ""] # [doc = " While it tracks the amount of usages using `u32`, we only ever"] # [doc = " care whether there are any. We only count them to be able to ignore"] # [doc = " usages from irrelevant candidates while evaluating a goal."] # [doc = ""] # [doc = " This cares about how nested goals relied on a cycle head. It does"] # [doc = " not care about how frequently the nested goal relied on it."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] struct HeadUsages { inductive : u32 , unknown : u32 , coinductive : u32 , forced_ambiguity : u32 , }
    };
}

HeadUsages!()