// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (not (target_os = "solana"))] impl core :: error :: Error for SanitizeMessageError { fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: IndexOutOfBounds => None , Self :: ValueOutOfBounds => None , Self :: InvalidValue => None , Self :: AddressLoaderError (e) => Some (e) , } } }
};
}
