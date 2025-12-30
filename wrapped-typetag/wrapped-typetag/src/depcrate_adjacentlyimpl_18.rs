// Generated macro for impl_18 (impl)
macro_rules! Depcrate_adjacentlyimpl_18 {
() => {
// Module: crate::adjacently
// Provides: {"impl_18"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for TagContentOtherFieldVisitor { type Value = TagContentOtherField ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
