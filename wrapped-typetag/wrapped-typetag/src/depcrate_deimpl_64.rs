// Generated macro for impl_64 (impl)
macro_rules! Depcrate_deimpl_64 {
() => {
// Module: crate::de
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'de , 'a , T : ? Sized + 'static > DeserializeSeed < 'de > for MapLookupVisitor < 'a , T > { type Value = DeserializeFn < T > ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
