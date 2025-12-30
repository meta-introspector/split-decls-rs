// Generated macro for impl_120 (impl)
macro_rules! Depcrate_decoding_errorsimpl_120 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_120"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for FSEDecoderError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { FSEDecoderError :: GetBitsError (source) => Some (source) , _ => None , } } }
};
}
