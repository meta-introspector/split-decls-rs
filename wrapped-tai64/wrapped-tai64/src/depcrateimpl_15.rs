// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for Tai64 { fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { Ok (< [u8 ; Tai64 :: BYTE_SIZE] > :: deserialize (deserializer) ? . into ()) } }
};
}
