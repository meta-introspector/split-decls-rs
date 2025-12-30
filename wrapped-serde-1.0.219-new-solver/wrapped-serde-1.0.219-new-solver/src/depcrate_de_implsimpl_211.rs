// Generated macro for impl_211 (impl)
macro_rules! Depcrate_de_implsimpl_211 {
() => {
// Module: crate::de::impls
// Provides: {"impl_211"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for Option < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_option (OptionVisitor { marker : PhantomData , }) } }
};
}
