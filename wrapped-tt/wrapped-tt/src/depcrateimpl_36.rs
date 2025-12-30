// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < S > Leaf < S > { pub fn span (& self) -> & S { match self { Leaf :: Literal (it) => & it . span , Leaf :: Punct (it) => & it . span , Leaf :: Ident (it) => & it . span , } } }
};
}
