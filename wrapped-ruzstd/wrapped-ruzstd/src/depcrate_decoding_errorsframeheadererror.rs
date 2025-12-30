// Generated macro for FrameHeaderError (enum)
macro_rules! Depcrate_decoding_errorsFrameHeaderError {
() => {
// Module: crate::decoding::errors
// Provides: {"FrameHeaderError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum FrameHeaderError { WindowTooBig { got : u64 } , WindowTooSmall { got : u64 } , FrameDescriptorError (FrameDescriptorError) , DictIdTooSmall { got : usize , expected : usize } , MismatchedFrameSize { got : usize , expected : u8 } , FrameSizeIsZero , InvalidFrameSize { got : u8 } , }
};
}
