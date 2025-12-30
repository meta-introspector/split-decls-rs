// Generated macro for impl_211 (impl)
macro_rules! Depcrate_serdeimpl_211 {
() => {
// Module: crate::serde
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Box < ByteStr > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { if deserializer . is_human_readable () { let s = deserializer . deserialize_any (ByteStrVisitor) ? ; Ok (ByteStr :: from_boxed_bytes (s)) } else { let s = Vec :: < u8 > :: deserialize (deserializer) ? ; Ok (ByteStr :: from_boxed_bytes (s . into_boxed_slice ())) } } }
};
}
