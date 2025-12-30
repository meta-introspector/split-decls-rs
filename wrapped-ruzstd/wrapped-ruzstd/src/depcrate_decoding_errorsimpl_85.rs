// Generated macro for impl_85 (impl)
macro_rules! Depcrate_decoding_errorsimpl_85 {
() => {
// Module: crate::decoding::errors
// Provides: {"impl_85"}
// Dependencies: {}
# [cfg (feature = "std")] impl StdError for FrameDecoderError { fn source (& self) -> Option < & (dyn StdError + 'static) > { match self { FrameDecoderError :: ReadFrameHeaderError (source) => Some (source) , FrameDecoderError :: FrameHeaderError (source) => Some (source) , FrameDecoderError :: DictionaryDecodeError (source) => Some (source) , FrameDecoderError :: FailedToReadBlockHeader (source) => Some (source) , FrameDecoderError :: FailedToReadBlockBody (source) => Some (source) , FrameDecoderError :: FailedToReadChecksum (source) => Some (source) , FrameDecoderError :: FailedToInitialize (source) => Some (source) , FrameDecoderError :: FailedToDrainDecodebuffer (source) => Some (source) , _ => None , } } }
};
}
