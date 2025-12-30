// Generated macro for impl_45 (impl)
macro_rules! Depcrateimpl_45 {
() => {
// Module: crate
// Provides: {"impl_45"}
// Dependencies: {}
impl < T > fmt :: Debug for IterMut < '_ , T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("IterMut") . field ("remaining" , & self . len) . finish () } }
};
}
