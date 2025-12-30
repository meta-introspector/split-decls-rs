// Generated macro for impl_108 (impl)
macro_rules! Depcrate_decoding_errorsimpl_108 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_108"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for LiteralsSectionParseError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { LiteralsSectionParseError :: GetBitsError (source) => Some (source) , _ => None , } } }
};
}
