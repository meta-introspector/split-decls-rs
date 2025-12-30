// Generated macro for impl_99 (impl)
macro_rules! Depcrate_decoding_errorsimpl_99 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_99"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for ExecuteSequencesError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { ExecuteSequencesError :: DecodebufferError (source) => Some (source) , _ => None , } } }
};
}
