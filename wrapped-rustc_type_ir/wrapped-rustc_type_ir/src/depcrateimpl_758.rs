// Generated macro for impl_758 (impl)
macro_rules! Depcrateimpl_758 {
() => {
// Module: crate
// Provides: {"impl_758"}
// Dependencies: {}
impl fmt :: Debug for Variance { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . write_str (match * self { Variance :: Covariant => "+" , Variance :: Contravariant => "-" , Variance :: Invariant => "o" , Variance :: Bivariant => "*" , }) } }
};
}
