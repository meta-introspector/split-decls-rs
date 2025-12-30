// Generated macro for ReadFrameHeaderError (enum)
macro_rules! Depcrate_decoding_errorsReadFrameHeaderError {
() => {
// Module: crate::decoding::errors
// Provides: {"ReadFrameHeaderError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum ReadFrameHeaderError { MagicNumberReadError (Error) , BadMagicNumber (u32) , FrameDescriptorReadError (Error) , InvalidFrameDescriptor (FrameDescriptorError) , WindowDescriptorReadError (Error) , DictionaryIdReadError (Error) , FrameContentSizeReadError (Error) , SkipFrame { magic_number : u32 , length : u32 } , }
};
}
