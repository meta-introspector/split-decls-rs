// Generated macro for impl_54 (impl)
macro_rules! Depcrate_seedimpl_54 {
() => {
// Module: crate::seed
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for & mut dyn ErasedDeserializeSeed < 'de > { type Value = ErasedValue ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { let deserializer = Box :: new (< dyn erased_serde :: Deserializer > :: erase (deserializer)) ; self . erased_deserialize (deserializer) . map_err (serde :: de :: Error :: custom) } }
};
}
