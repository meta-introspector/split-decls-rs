// Generated macro for impl_58 (impl)
macro_rules! Depcrate_fmtimpl_58 {
() => {
// Module: crate::fmt
// Provides: {"impl_58"}
// Dependencies: {}
impl fmt :: Display for Variant { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match * self { Variant :: NCS => write ! (f , "NCS") , Variant :: RFC4122 => write ! (f , "RFC4122") , Variant :: Microsoft => write ! (f , "Microsoft") , Variant :: Future => write ! (f , "Future") , } } }
};
}
