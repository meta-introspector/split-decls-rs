macro_rules! macro_155 {
    () => {
        decl_derive ! ([HashStable_NoContext] => # [doc = " `HashStable` implementation that has no `HashStableContext` bound and"] # [doc = " which adds `where` bounds for `HashStable` based off of fields and not"] # [doc = " generics. This is suitable for use in crates like `rustc_type_ir`."] hash_stable :: hash_stable_no_context_derive) ;
    };
}

macro_155!()