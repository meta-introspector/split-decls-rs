// Generated macro for impl_87 (impl)
macro_rules! Depcrate_deimpl_87 {
() => {
// Module: crate::de
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'de , const N : usize > Deserialize < 'de > for ByteArray < N > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) } }
};
}
