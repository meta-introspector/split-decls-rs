// Generated macro for impl_66 (impl)
macro_rules! Depcrate_deimpl_66 {
() => {
// Module: crate::de
// Provides: {"impl_66"}
// Dependencies: {}
impl < 'de , T : ? Sized > DeserializeSeed < 'de > for FnApply < T > { type Value = Box < T > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { let mut erased = < dyn erased_serde :: Deserializer > :: erase (deserializer) ; (self . deserialize_fn) (& mut erased) . map_err (de :: Error :: custom) } }
};
}
