// Generated macro for impl_116 (impl)
macro_rules! Depcrate_decoding_errorsimpl_116 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_116"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for FSETableError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { FSETableError :: GetBitsError (source) => Some (source) , _ => None , } } }
};
}
