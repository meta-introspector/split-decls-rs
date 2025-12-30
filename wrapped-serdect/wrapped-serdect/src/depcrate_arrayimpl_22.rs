// Generated macro for impl_22 (impl)
macro_rules! Depcrate_arrayimpl_22 {
() => {
// Module: crate::array
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'de , const N : usize , const UPPERCASE : bool > Deserialize < 'de > for HexOrBin < N , UPPERCASE > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let mut buffer = [0 ; N] ; deserialize_hex_or_bin (& mut buffer , deserializer) ? ; Ok (Self (buffer)) } }
};
}
