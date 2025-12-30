// Generated macro for impl_73 (impl)
macro_rules! Depcrate_decoding_errorsimpl_73 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_73"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for DecodeBlockContentError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecodeBlockContentError :: ReadError { step : _ , source } => Some (source) , DecodeBlockContentError :: DecompressBlockError (source) => Some (source) , _ => None , } } }
};
}
