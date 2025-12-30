// Generated macro for DecodeSequenceError (enum)
macro_rules! Depcrate_decoding_errorsDecodeSequenceError {
() => {
// Module: crate::decoding::errors
// Provides: {"DecodeSequenceError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DecodeSequenceError { GetBitsError (GetBitsError) , FSEDecoderError (FSEDecoderError) , FSETableError (FSETableError) , ExtraPadding { skipped_bits : i32 } , UnsupportedOffset { offset_code : u8 } , ZeroOffset , NotEnoughBytesForNumSequences , ExtraBits { bits_remaining : isize } , MissingCompressionMode , MissingByteForRleLlTable , MissingByteForRleOfTable , MissingByteForRleMlTable , }
};
}
