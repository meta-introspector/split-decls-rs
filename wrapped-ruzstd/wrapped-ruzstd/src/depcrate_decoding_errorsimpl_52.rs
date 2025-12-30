// Generated macro for impl_52 (impl)
macro_rules! Depcrate_decoding_errorsimpl_52 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_52"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for BlockHeaderReadError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { BlockHeaderReadError :: ReadError (source) => Some (source) , BlockHeaderReadError :: BlockTypeError (source) => Some (source) , BlockHeaderReadError :: BlockSizeError (source) => Some (source) , BlockHeaderReadError :: FoundReservedBlock => None , } } }
};
}
