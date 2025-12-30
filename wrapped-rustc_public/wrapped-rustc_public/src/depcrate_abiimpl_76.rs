// Generated macro for impl_76 (impl)
macro_rules! Depcrate_abiimpl_76 {
() => {
// Module: crate::abi
// Provides: {"impl_76"}
// Dependencies: {}
impl Debug for WrappingRange { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { if self . start > self . end { write ! (fmt , "(..={}) | ({}..)" , self . end , self . start) ? ; } else { write ! (fmt , "{}..={}" , self . start , self . end) ? ; } Ok (()) } }
};
}
