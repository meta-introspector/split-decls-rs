// Generated macro for impl_124 (impl)
macro_rules! Depcrate_decoding_errorsimpl_124 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_124"}
// Dependencies: {}
# [cfg (feature = "std")] impl StdError for HuffmanTableError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { HuffmanTableError :: GetBitsError (source) => Some (source) , HuffmanTableError :: FSEDecoderError (source) => Some (source) , HuffmanTableError :: FSETableError (source) => Some (source) , _ => None , } } }
};
}
