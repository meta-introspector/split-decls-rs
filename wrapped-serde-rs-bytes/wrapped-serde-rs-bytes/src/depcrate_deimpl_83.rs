// Generated macro for impl_83 (impl)
macro_rules! Depcrate_deimpl_83 {
() => {
// Module: crate::de
// Provides: {"impl_83"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de > Deserialize < 'de > for Vec < u8 > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (ByteBuf :: into_vec) } }
};
}
