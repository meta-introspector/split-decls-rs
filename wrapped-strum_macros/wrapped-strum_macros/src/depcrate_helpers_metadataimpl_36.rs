// Generated macro for impl_36 (impl)
macro_rules! Depcrate_helpers_metadataimpl_36 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_36"}
// Dependencies: {}
impl DeriveInputExt for DeriveInput { fn get_metadata (& self) -> syn :: Result < Vec < EnumMeta > > { get_metadata_inner ("strum" , & self . attrs) } fn get_discriminants_metadata (& self) -> syn :: Result < Vec < EnumDiscriminantsMeta > > { get_metadata_inner ("strum_discriminants" , & self . attrs) } }
};
}
