// Generated macro for BlockHeaderReadError (enum)
macro_rules! Depcrate_decoding_errorsBlockHeaderReadError {
() => {
// Module: crate::decoding::errors
// Provides: {"BlockHeaderReadError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum BlockHeaderReadError { ReadError (Error) , FoundReservedBlock , BlockTypeError (BlockTypeError) , BlockSizeError (BlockSizeError) , }
};
}
