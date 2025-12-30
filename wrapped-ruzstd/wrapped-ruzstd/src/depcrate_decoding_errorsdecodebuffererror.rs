// Generated macro for DecodeBufferError (enum)
macro_rules! Depcrate_decoding_errorsDecodeBufferError {
() => {
// Module: crate::decoding::errors
// Provides: {"DecodeBufferError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DecodeBufferError { NotEnoughBytesInDictionary { got : usize , need : usize } , OffsetTooBig { offset : usize , buf_len : usize } , }
};
}
