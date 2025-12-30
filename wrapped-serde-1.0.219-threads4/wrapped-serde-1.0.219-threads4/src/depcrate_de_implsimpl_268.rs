// Generated macro for impl_268 (impl)
macro_rules! Depcrate_de_implsimpl_268 {
() => {
// Module: crate::de::impls
// Provides: {"impl_268"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Cell < T > where T : Deserialize < 'de > + Copy , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { T :: deserialize (deserializer) . map (Cell :: new) } }
};
}
