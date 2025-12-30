// Generated macro for impl_45 (impl)
macro_rules! Depcrate_providerimpl_45 {
() => {
// Module: crate::provider
// Provides: {"impl_45"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for VariantOffsets { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; if deserializer . is_human_readable () { let raw = < & str > :: deserialize (deserializer) ? ; Ok (if let Some ((std , dst)) = raw . split_once ('/') { Self { standard : UtcOffset :: try_from_str (std) . map_err (| _ | D :: Error :: custom ("invalid offset")) ? , daylight : Some (UtcOffset :: try_from_str (dst) . map_err (| _ | D :: Error :: custom ("invalid offset")) ? ,) , } } else { Self { standard : UtcOffset :: try_from_str (raw) . map_err (| _ | D :: Error :: custom ("invalid offset")) ? , daylight : None , } }) } else { < _ > :: deserialize (deserializer) . map (Self :: from_unaligned) } } }
};
}
