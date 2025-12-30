// Generated macro for impl_219 (impl)
macro_rules! Depcrate_de_deserializerimpl_219 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_219"}
// Dependencies: {}
impl < 'i > From < Spanned < DeTable < 'i > > > for Deserializer < 'i > { fn from (root : Spanned < DeTable < 'i > >) -> Self { let span = root . span () ; let root = root . into_inner () ; Self { span , root , raw : None , } } }
};
}
