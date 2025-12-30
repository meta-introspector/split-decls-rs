// Generated macro for impl_369 (impl)
macro_rules! Depcrate_x509impl_369 {
() => {
// Module: crate::x509
// Provides: {"impl_369"}
// Dependencies: {}
impl Extension < '_ > { pub (crate) fn unsupported (& self) -> Result < () , Error > { match self . critical { true => Err (Error :: UnsupportedCriticalExtension) , false => Ok (()) , } } }
};
}
