// Generated macro for impl_10 (impl)
macro_rules! Depcrate_extimpl_10 {
() => {
// Module: crate::ext
// Provides: {"impl_10"}
// Dependencies: {}
impl DataExt for DataUnion { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { map_fields (& self . fields . named) } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { vec ! [self . fields ()] } fn tag (& self) -> Option < Ident > { None } }
};
}
