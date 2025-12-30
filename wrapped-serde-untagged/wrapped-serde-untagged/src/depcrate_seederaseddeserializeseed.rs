// Generated macro for ErasedDeserializeSeed (trait)
macro_rules! Depcrate_seedErasedDeserializeSeed {
() => {
// Module: crate::seed
// Provides: {"ErasedDeserializeSeed"}
// Dependencies: {}
pub (crate) trait ErasedDeserializeSeed < 'de > { fn erased_deserialize (& mut self , deserializer : Box < dyn erased_serde :: Deserializer < 'de > + '_ > ,) -> Result < ErasedValue , erased_serde :: Error > ; }
};
}
