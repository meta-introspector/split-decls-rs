// Generated macro for impl_177 (impl)
macro_rules! Depcrate_litimpl_177 {
() => {
// Module: crate::lit
// Provides: {"impl_177"}
// Dependencies: {}
impl fmt :: Debug for LitKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { LitKind :: Bool (b) => b . fmt (f) , LitKind :: Other (ref l) => fmt :: Display :: fmt (l , f) , } } }
};
}
