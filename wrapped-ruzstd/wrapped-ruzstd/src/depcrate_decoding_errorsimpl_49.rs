// Generated macro for impl_49 (impl)
macro_rules! Depcrate_decoding_errorsimpl_49 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_49"}
// Dependencies: {}
# [cfg (feature = "std")] impl StdError for ReadFrameHeaderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { ReadFrameHeaderError :: MagicNumberReadError (source) => Some (source) , ReadFrameHeaderError :: FrameDescriptorReadError (source) => Some (source) , ReadFrameHeaderError :: InvalidFrameDescriptor (source) => Some (source) , ReadFrameHeaderError :: WindowDescriptorReadError (source) => Some (source) , ReadFrameHeaderError :: DictionaryIdReadError (source) => Some (source) , ReadFrameHeaderError :: FrameContentSizeReadError (source) => Some (source) , _ => None , } } }
};
}
