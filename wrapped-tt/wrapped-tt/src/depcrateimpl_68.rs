// Generated macro for impl_68 (impl)
macro_rules! Depcrateimpl_68 {
() => {
// Module: crate
// Provides: {"impl_68"}
// Dependencies: {}
impl < S > fmt :: Display for Leaf < S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Leaf :: Ident (it) => fmt :: Display :: fmt (it , f) , Leaf :: Literal (it) => fmt :: Display :: fmt (it , f) , Leaf :: Punct (it) => fmt :: Display :: fmt (it , f) , } } }
};
}
