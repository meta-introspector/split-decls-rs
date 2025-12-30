// Generated macro for HuffmanTableError (enum)
macro_rules! Depcrate_decoding_errorsHuffmanTableError {
() => {
// Module: crate::decoding::errors
// Provides: {"HuffmanTableError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum HuffmanTableError { GetBitsError (GetBitsError) , FSEDecoderError (FSEDecoderError) , FSETableError (FSETableError) , SourceIsEmpty , NotEnoughBytesForWeights { got_bytes : usize , expected_bytes : u8 , } , ExtraPadding { skipped_bits : i32 , } , TooManyWeights { got : usize , } , MissingWeights , LeftoverIsNotAPowerOf2 { got : u32 , } , NotEnoughBytesToDecompressWeights { have : usize , need : usize , } , FSETableUsedTooManyBytes { used : usize , available_bytes : u8 , } , NotEnoughBytesInSource { got : usize , need : usize , } , WeightBiggerThanMaxNumBits { got : u8 , } , MaxBitsTooHigh { got : u8 , } , }
};
}
