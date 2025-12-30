// Generated macro for impl_53 (impl)
macro_rules! Depcrate_seedimpl_53 {
() => {
// Module: crate::seed
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de , Seed > ErasedDeserializeSeed < 'de > for Option < Seed > where Seed : DeserializeSeed < 'de > , { fn erased_deserialize (& mut self , deserializer : Box < dyn erased_serde :: Deserializer < 'de > + '_ > ,) -> Result < ErasedValue , erased_serde :: Error > { self . take () . unwrap () . deserialize (deserializer) . map (| value | unsafe { ErasedValue :: new :: < Seed :: Value > (value) }) } }
};
}
