// Generated macro for impl_346 (impl)
macro_rules! Depcrate_providerimpl_346 {
() => {
// Module: crate::provider
// Provides: {"impl_346"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for BreakState { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { Ok (zerovec :: ule :: AsULE :: from_unaligned (i8 :: deserialize (deserializer) ? . to_le_bytes () [0] ,)) } else { u8 :: deserialize (deserializer) . map (zerovec :: ule :: AsULE :: from_unaligned) } } }
};
}
