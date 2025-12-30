// Generated macro for impl_89 (impl)
macro_rules! Depcrate_deimpl_89 {
() => {
// Module: crate::de
// Provides: {"impl_89"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de > Deserialize < 'de > for ByteBuf { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) } }
};
}
