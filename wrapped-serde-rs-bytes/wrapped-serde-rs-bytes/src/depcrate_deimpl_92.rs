// Generated macro for impl_92 (impl)
macro_rules! Depcrate_deimpl_92 {
() => {
// Module: crate::de
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de > Deserialize < 'de > for Box < [u8] > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { Deserialize :: deserialize (deserializer) . map (Vec :: into_boxed_slice) } }
};
}
