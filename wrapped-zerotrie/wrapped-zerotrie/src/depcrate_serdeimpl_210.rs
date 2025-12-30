// Generated macro for impl_210 (impl)
macro_rules! Depcrate_serdeimpl_210 {
() => {
// Module: crate::serde
// Provides: {"impl_210"}
// Dependencies: {}
impl < 'data , 'de : 'data > Deserialize < 'de > for & 'data ByteStr { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let s = < & 'data [u8] > :: deserialize (deserializer) ? ; Ok (ByteStr :: from_bytes (s)) } }
};
}
