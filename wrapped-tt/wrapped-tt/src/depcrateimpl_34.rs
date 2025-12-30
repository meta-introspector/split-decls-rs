// Generated macro for impl_34 (impl)
macro_rules! Depcrateimpl_34 {
() => {
// Module: crate
// Provides: {"impl_34"}
// Dependencies: {}
impl < S : Copy > TokenTree < S > { pub fn first_span (& self) -> S { match self { TokenTree :: Leaf (l) => * l . span () , TokenTree :: Subtree (s) => s . delimiter . open , } } }
};
}
