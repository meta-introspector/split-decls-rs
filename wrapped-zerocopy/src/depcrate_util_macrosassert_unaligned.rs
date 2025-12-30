// Generated macro for assert_unaligned (macro)
macro_rules! Depcrate_util_macrosassert_unaligned {
() => {
// Module: crate::util::macros
// Provides: {"assert_unaligned"}
// Dependencies: {}
# [doc = " Uses `align_of` to confirm that a type or set of types have alignment 1."] # [doc = ""] # [doc = " Note that `align_of<T>` requires `T: Sized`, so this macro doesn't work for"] # [doc = " unsized types."] macro_rules ! assert_unaligned { ($ ($ tys : ty) ,*) => { $ (# [cfg (test)] static_assertions :: const_assert_eq ! (core :: mem :: align_of ::<$ tys > () , 1) ;) * } ; }
};
}
