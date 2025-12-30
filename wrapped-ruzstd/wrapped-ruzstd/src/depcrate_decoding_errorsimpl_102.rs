// Generated macro for impl_102 (impl)
macro_rules! Depcrate_decoding_errorsimpl_102 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_102"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for DecodeSequenceError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecodeSequenceError :: GetBitsError (source) => Some (source) , DecodeSequenceError :: FSEDecoderError (source) => Some (source) , DecodeSequenceError :: FSETableError (source) => Some (source) , _ => None , } } }
};
}
