// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl < T > fmt :: Debug for Iter < '_ , T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("Iter") . field ("remaining" , & self . len) . finish () } }
};
}
