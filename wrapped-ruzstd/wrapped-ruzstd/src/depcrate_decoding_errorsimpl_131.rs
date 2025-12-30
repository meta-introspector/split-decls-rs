// Generated macro for impl_131 (impl)
macro_rules! Depcrate_decoding_errorsimpl_131 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_131"}
// Dependencies: {}
# [cfg (feature = "std")] impl StdError for HuffmanDecoderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { HuffmanDecoderError :: GetBitsError (source) => Some (source) , } } }
};
}
