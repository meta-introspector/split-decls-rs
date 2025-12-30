// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl crypto :: SupportedKxGroup for KeyExchangeGroup { fn start (& self) -> Result < StartedKeyExchange , Error > { Ok (StartedKeyExchange :: Single (Box :: new (ActiveKeyExchange))) } fn name (& self) -> NamedGroup { NamedGroup :: from (0xfe00) } }
};
}
