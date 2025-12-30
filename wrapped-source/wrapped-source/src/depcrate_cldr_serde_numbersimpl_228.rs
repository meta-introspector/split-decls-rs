// Generated macro for impl_228 (impl)
macro_rules! Depcrate_cldr_serde_numbersimpl_228 {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"impl_228"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for DecimalFormat { fn deserialize < D > (deserializer : D) -> Result < DecimalFormat , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (DecimalFormatVisitor) } }
};
}
