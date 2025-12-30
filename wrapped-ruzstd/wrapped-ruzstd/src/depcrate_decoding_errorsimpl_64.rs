// Generated macro for impl_64 (impl)
macro_rules! Depcrate_decoding_errorsimpl_64 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for DecompressBlockError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self { DecompressBlockError :: BlockContentReadError (source) => Some (source) , DecompressBlockError :: DecompressLiteralsError (source) => Some (source) , DecompressBlockError :: LiteralsSectionParseError (source) => Some (source) , DecompressBlockError :: SequencesHeaderParseError (source) => Some (source) , DecompressBlockError :: DecodeSequenceError (source) => Some (source) , DecompressBlockError :: ExecuteSequencesError (source) => Some (source) , _ => None , } } }
};
}
