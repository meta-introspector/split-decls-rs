// Generated macro for impl_47 (impl)
macro_rules! Depcrate_helpers_metadataimpl_47 {
() => {
// Module: crate::helpers::metadata
// Provides: {"impl_47"}
// Dependencies: {}
impl InnerVariantExt for Field { fn get_named_metadata (& self) -> syn :: Result < Vec < InnerVariantMeta > > { let result = get_metadata_inner ("strum" , & self . attrs) ? ; self . attrs . iter () . filter (| attr | attr . meta . path () . is_ident ("default_with")) . try_fold (result , | vec , _attr | Ok (vec)) } }
};
}
