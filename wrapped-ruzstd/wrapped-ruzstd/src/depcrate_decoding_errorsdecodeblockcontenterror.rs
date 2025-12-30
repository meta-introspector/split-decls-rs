// Generated macro for DecodeBlockContentError (enum)
macro_rules! Depcrate_decoding_errorsDecodeBlockContentError {
() => {
// Module: crate::decoding::errors
// Provides: {"DecodeBlockContentError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DecodeBlockContentError { DecoderStateIsFailed , ExpectedHeaderOfPreviousBlock , ReadError { step : BlockType , source : Error } , DecompressBlockError (DecompressBlockError) , }
};
}
