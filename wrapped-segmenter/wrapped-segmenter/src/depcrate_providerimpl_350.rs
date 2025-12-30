// Generated macro for impl_350 (impl)
macro_rules! Depcrate_providerimpl_350 {
() => {
// Module: crate::provider
// Provides: {"impl_350"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for WordType { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { if deserializer . is_human_readable () { use serde :: de :: Error ; match u8 :: deserialize (deserializer) { Ok (0) => Ok (WordType :: None) , Ok (1) => Ok (WordType :: Number) , Ok (2) => Ok (WordType :: Letter) , Ok (_) => Err (D :: Error :: custom ("invalid value")) , Err (e) => Err (e) , } } else { unreachable ! ("only used as ULE") } } }
};
}
