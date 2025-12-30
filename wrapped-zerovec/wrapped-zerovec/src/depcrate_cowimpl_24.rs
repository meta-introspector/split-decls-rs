// Generated macro for impl_24 (impl)
macro_rules! Depcrate_cowimpl_24 {
() => {
// Module: crate::cow
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'a , V : VarULE + ? Sized + fmt :: Debug > fmt :: Debug for VarZeroCow < 'a , V > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> Result < () , fmt :: Error > { self . deref () . fmt (f) } }
};
}
