// Generated macro for impl_382 (impl)
macro_rules! Depcrate_canonicalimpl_382 {
() => {
// Module: crate::canonical
// Provides: {"impl_382"}
// Dependencies: {}
impl < I : Interner , V > Canonical < I , V > { # [doc = " Allows you to map the `value` of a canonical while keeping the"] # [doc = " same set of bound variables."] # [doc = ""] # [doc = " **WARNING:** This function is very easy to mis-use, hence the"] # [doc = " name!  In particular, the new value `W` must use all **the"] # [doc = " same type/region variables** in **precisely the same order**"] # [doc = " as the original! (The ordering is defined by the"] # [doc = " `TypeFoldable` implementation of the type in question.)"] # [doc = ""] # [doc = " An example of a **correct** use of this:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let a: Canonical<I, T> = ...;"] # [doc = " let b: Canonical<I, (T,)> = a.unchecked_map(|v| (v, ));"] # [doc = " ```"] # [doc = ""] # [doc = " An example of an **incorrect** use of this:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let a: Canonical<I, T> = ...;"] # [doc = " let ty: Ty<I> = ...;"] # [doc = " let b: Canonical<I, (T, Ty<I>)> = a.unchecked_map(|v| (v, ty));"] # [doc = " ```"] pub fn unchecked_map < W > (self , map_op : impl FnOnce (V) -> W) -> Canonical < I , W > { let Canonical { max_universe , variables , value } = self ; Canonical { max_universe , variables , value : map_op (value) } } }
};
}
