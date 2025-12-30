// Generated macro for impl_20 (impl)
macro_rules! Depcrateimpl_20 {
() => {
// Module: crate
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for ShortVec < T > { fn deserialize < D > (deserializer : D) -> Result < ShortVec < T > , D :: Error > where D : Deserializer < 'de > , { deserialize (deserializer) . map (ShortVec) } }
};
}
