// Generated macro for FrameDecoderError (enum)
macro_rules! Depcrate_decoding_errorsFrameDecoderError {
() => {
// Module: crate::decoding::errors
// Provides: {"FrameDecoderError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum FrameDecoderError { ReadFrameHeaderError (ReadFrameHeaderError) , FrameHeaderError (FrameHeaderError) , WindowSizeTooBig { requested : u64 } , DictionaryDecodeError (DictionaryDecodeError) , FailedToReadBlockHeader (BlockHeaderReadError) , FailedToReadBlockBody (DecodeBlockContentError) , FailedToReadChecksum (Error) , NotYetInitialized , FailedToInitialize (FrameHeaderError) , FailedToDrainDecodebuffer (Error) , FailedToSkipFrame , TargetTooSmall , DictNotProvided { dict_id : u32 } , }
};
}
