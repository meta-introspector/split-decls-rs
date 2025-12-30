// Generated macro for impl_185 (impl)
macro_rules! Depcrate_readimpl_185 {
() => {
// Module: crate::read
// Provides: {"impl_185"}
// Dependencies: {}
# [doc = " Methods for retrieving information on zip files"] impl < R : Read > ZipFile < '_ , R > { # [doc = " iterate through all extra fields"] pub fn extra_data_fields (& self) -> impl Iterator < Item = & ExtraField > { self . data . extra_fields . iter () } }
};
}
