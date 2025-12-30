// Generated macro for impl_85 (impl)
macro_rules! Depcrateimpl_85 {
() => {
// Module: crate
// Provides: {"impl_85"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for IntoIter < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_tuple ("IntoIter") . field (& self . as_slice ()) . finish () } }
};
}
