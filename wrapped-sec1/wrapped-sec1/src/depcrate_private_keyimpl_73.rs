// Generated macro for impl_73 (impl)
macro_rules! Depcrate_private_keyimpl_73 {
() => {
// Module: crate::private_key
// Provides: {"impl_73"}
// Dependencies: {}
impl fmt :: Debug for EcPrivateKey < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("EcPrivateKey") . field ("parameters" , & self . parameters) . field ("public_key" , & self . public_key) . finish_non_exhaustive () } }
};
}
