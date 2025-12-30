// Generated macro for impl_93 (impl)
macro_rules! Depcrate_deimpl_93 {
() => {
// Module: crate::de
// Provides: {"impl_93"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de > Deserialize < 'de > for Box < Bytes > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let bytes : Box < [u8] > = Deserialize :: deserialize (deserializer) ? ; Ok (bytes . into ()) } }
};
}
