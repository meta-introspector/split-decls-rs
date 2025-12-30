// Generated macro for impl_201 (impl)
macro_rules! Depcrate_de_deserializer_valueimpl_201 {
() => {
// Module: crate::de::deserializer::value
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'i > From < Spanned < DeValue < 'i > > > for ValueDeserializer < 'i > { fn from (root : Spanned < DeValue < 'i > >) -> Self { let span = root . span () ; let root = root . into_inner () ; Self :: with_parts (root , span) } }
};
}
