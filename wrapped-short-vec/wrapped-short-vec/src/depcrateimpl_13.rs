// Generated macro for impl_13 (impl)
macro_rules! Depcrateimpl_13 {
() => {
// Module: crate
// Provides: {"impl_13"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for ShortU16 { fn deserialize < D > (deserializer : D) -> Result < ShortU16 , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_tuple (3 , ShortU16Visitor) } }
};
}
