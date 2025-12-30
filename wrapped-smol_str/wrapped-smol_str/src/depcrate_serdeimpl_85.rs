// Generated macro for impl_85 (impl)
macro_rules! Depcrate_serdeimpl_85 {
() => {
// Module: crate::serde
// Provides: {"impl_85"}
// Dependencies: {}
impl < 'de > serde :: Deserialize < 'de > for SmolStr { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { smol_str (deserializer) } }
};
}
