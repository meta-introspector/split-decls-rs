// Generated macro for cast (macro)
macro_rules! Depcrate_util_macroscast {
() => {
// Module: crate::util::macros
// Provides: {"cast"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that the cast does not grow the size of the referent."] # [doc = " Preserving or shrinking the size of the referent are both acceptable."] macro_rules ! cast { ($ p : expr) => { { let ptr : crate :: pointer :: PtrInner <'_ , _ > = $ p ; let ptr = ptr . as_non_null () ; let ptr = ptr . as_ptr () ; # [allow (clippy :: as_conversions)] let ptr = ptr as * mut _ ; # [allow (unused_unsafe)] let ptr = unsafe { core :: ptr :: NonNull :: new_unchecked (ptr) } ; crate :: pointer :: PtrInner :: new (ptr) } } ; }
};
}
