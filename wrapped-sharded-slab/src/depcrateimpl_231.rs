// Generated macro for impl_231 (impl)
macro_rules! Depcrateimpl_231 {
() => {
// Module: crate
// Provides: {"impl_231"}
// Dependencies: {}
impl < T , C > fmt :: Debug for OwnedEntry < T , C > where T : fmt :: Debug , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
