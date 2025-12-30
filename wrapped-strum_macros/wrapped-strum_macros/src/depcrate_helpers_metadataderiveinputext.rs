// Generated macro for DeriveInputExt (trait)
macro_rules! Depcrate_helpers_metadataDeriveInputExt {
() => {
// Module: crate::helpers::metadata
// Provides: {"DeriveInputExt"}
// Dependencies: {}
pub trait DeriveInputExt { # [doc = " Get all the strum metadata associated with an enum."] fn get_metadata (& self) -> syn :: Result < Vec < EnumMeta > > ; # [doc = " Get all the `strum_discriminants` metadata associated with an enum."] fn get_discriminants_metadata (& self) -> syn :: Result < Vec < EnumDiscriminantsMeta > > ; }
};
}
