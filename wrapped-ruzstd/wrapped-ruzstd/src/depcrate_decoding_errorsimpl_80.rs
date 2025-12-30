// Generated macro for impl_80 (impl)
macro_rules! Depcrate_decoding_errorsimpl_80 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_80"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for DictionaryDecodeError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DictionaryDecodeError :: FSETableError (source) => Some (source) , DictionaryDecodeError :: HuffmanTableError (source) => Some (source) , _ => None , } } }
};
}
