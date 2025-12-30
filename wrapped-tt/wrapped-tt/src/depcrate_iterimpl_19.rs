// Generated macro for impl_19 (impl)
macro_rules! Depcrate_iterimpl_19 {
() => {
// Module: crate::iter
// Provides: {"impl_19"}
// Dependencies: {}
impl < S : Copy > TtElement < '_ , S > { # [inline] pub fn first_span (& self) -> S { match self { TtElement :: Leaf (it) => * it . span () , TtElement :: Subtree (it , _) => it . delimiter . open , } } }
};
}
