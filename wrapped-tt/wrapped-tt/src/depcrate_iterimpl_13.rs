// Generated macro for impl_13 (impl)
macro_rules! Depcrate_iterimpl_13 {
() => {
// Module: crate::iter
// Provides: {"impl_13"}
// Dependencies: {}
impl < S : Copy + fmt :: Debug > fmt :: Debug for TtIter < '_ , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("TtIter") . field ("remaining" , & self . remaining ()) . finish () } }
};
}
