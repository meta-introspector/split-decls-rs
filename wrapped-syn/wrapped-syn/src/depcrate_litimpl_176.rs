// Generated macro for impl_176 (impl)
macro_rules! Depcrate_litimpl_176 {
() => {
// Module: crate::lit
// Provides: {"impl_176"}
// Dependencies: {}
impl fmt :: Display for LitKind { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { LitKind :: Bool (b) => b . fmt (f) , LitKind :: Other (ref l) => l . fmt (f) , } } }
};
}
