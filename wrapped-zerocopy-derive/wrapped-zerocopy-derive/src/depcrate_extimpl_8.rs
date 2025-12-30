// Generated macro for impl_8 (impl)
macro_rules! Depcrate_extimpl_8 {
() => {
// Module: crate::ext
// Provides: {"impl_8"}
// Dependencies: {}
impl DataExt for DataStruct { fn fields (& self) -> Vec < (& Visibility , TokenStream , & Type) > { map_fields (& self . fields) } fn variants (& self) -> Vec < Vec < (& Visibility , TokenStream , & Type) > > { vec ! [self . fields ()] } fn tag (& self) -> Option < Ident > { None } }
};
}
