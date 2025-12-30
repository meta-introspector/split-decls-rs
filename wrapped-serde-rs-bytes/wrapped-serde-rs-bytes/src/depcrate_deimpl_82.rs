// Generated macro for impl_82 (impl)
macro_rules! Depcrate_deimpl_82 {
() => {
// Module: crate::de
// Provides: {"impl_82"}
// Dependencies: {}
impl < 'de : 'a , 'a > Deserialize < 'de > for & 'a [u8] { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) } }
};
}
