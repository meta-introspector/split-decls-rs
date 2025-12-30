// Generated macro for impl_52 (impl)
macro_rules! Depcrate_providerimpl_52 {
() => {
// Module: crate::provider
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for TimezonePeriods < 'de > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { use serde :: de :: Error ; if deserializer . is_human_readable () { Err (D :: Error :: custom ("not yet supported; see icu4x#6752")) } else { let TimeZonePeriodsSerde { index , list , offsets , } = TimeZonePeriodsSerde :: deserialize (deserializer) ? ; Ok (Self { index , list , offsets , }) } } }
};
}
