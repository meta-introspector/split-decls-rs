// Generated macro for impl_209 (impl)
macro_rules! Depcrate_cldr_serde_locale_resourceimpl_209 {
() => {
// Module: crate::cldr_serde::locale_resource
// Provides: {"impl_209"}
// Dependencies: {}
impl < 'de , T > Deserialize < 'de > for SingleLocaleMap < T > where T : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let visitor = SingleLocaleMapVisitor (PhantomData) ; deserializer . deserialize_map (visitor) } }
};
}
