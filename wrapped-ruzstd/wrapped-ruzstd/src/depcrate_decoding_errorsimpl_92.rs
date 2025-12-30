// Generated macro for impl_92 (impl)
macro_rules! Depcrate_decoding_errorsimpl_92 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_92"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for DecompressLiteralsError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecompressLiteralsError :: GetBitsError (source) => Some (source) , DecompressLiteralsError :: HuffmanTableError (source) => Some (source) , DecompressLiteralsError :: HuffmanDecoderError (source) => Some (source) , _ => None , } } }
};
}
