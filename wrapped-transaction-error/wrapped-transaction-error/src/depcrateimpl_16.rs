// Generated macro for impl_16 (impl)
macro_rules! Depcrateimpl_16 {
() => {
// Module: crate
// Provides: {"impl_16"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl fmt :: Display for SanitizeMessageError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { Self :: IndexOutOfBounds => f . write_str ("index out of bounds") , Self :: ValueOutOfBounds => f . write_str ("value out of bounds") , Self :: InvalidValue => f . write_str ("invalid value") , Self :: AddressLoaderError (e) => { write ! (f , "{e}") } } } }
};
}
