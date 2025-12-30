// Generated macro for impl_31 (impl)
macro_rules! Depcrate_contentimpl_31 {
() => {
// Module: crate::content
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Content < 'de > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { let visitor = ContentVisitor { value : PhantomData } ; deserializer . deserialize_any (visitor) } }
};
}
