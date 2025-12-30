// Generated macro for impl_345 (impl)
macro_rules! Depcrate_providerimpl_345 {
() => {
// Module: crate::provider
// Provides: {"impl_345"}
// Dependencies: {}
# [cfg (feature = "datagen")] impl serde :: Serialize for BreakState { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : serde :: Serializer , { if serializer . is_human_readable () { i8 :: from_le_bytes ([zerovec :: ule :: AsULE :: to_unaligned (* self)]) . serialize (serializer) } else { zerovec :: ule :: AsULE :: to_unaligned (* self) . serialize (serializer) } } }
};
}
