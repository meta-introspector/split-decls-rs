// Generated macro for impl_224 (impl)
macro_rules! Depcrateimpl_224 {
() => {
// Module: crate
// Provides: {"impl_224"}
// Dependencies: {}
impl < T , C > fmt :: Debug for Entry < '_ , T , C > where T : fmt :: Debug , C : cfg :: Config , { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . value () , f) } }
};
}
