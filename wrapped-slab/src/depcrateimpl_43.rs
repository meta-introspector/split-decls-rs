// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < T > fmt :: Debug for IntoIter < T > where T : fmt :: Debug , { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt . debug_struct ("IntoIter") . field ("remaining" , & self . len) . finish () } }
};
}
