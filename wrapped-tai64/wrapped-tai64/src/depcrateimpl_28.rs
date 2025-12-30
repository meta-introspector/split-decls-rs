// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > Deserialize < 'de > for Tai64N { fn deserialize < D : de :: Deserializer < 'de > > (deserializer : D) -> Result < Self , D :: Error > { use de :: Error ; < [u8 ; Tai64N :: BYTE_SIZE] > :: deserialize (deserializer) ? . try_into () . map_err (D :: Error :: custom) } }
};
}
