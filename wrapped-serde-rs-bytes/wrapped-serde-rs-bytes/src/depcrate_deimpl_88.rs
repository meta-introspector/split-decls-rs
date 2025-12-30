// Generated macro for impl_88 (impl)
macro_rules! Depcrate_deimpl_88 {
() => {
// Module: crate::de
// Provides: {"impl_88"}
// Dependencies: {}
impl < 'de : 'a , 'a , const N : usize > Deserialize < 'de > for & 'a ByteArray < N > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) } }
};
}
