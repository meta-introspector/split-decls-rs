// Generated macro for impl_348 (impl)
macro_rules! Depcrate_providerimpl_348 {
() => {
// Module: crate::provider
// Provides: {"impl_348"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl serde :: Serialize for WordType { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { (* self as u8) . serialize (serializer) } else { unreachable ! ("only used as ULE") } } }
};
}
