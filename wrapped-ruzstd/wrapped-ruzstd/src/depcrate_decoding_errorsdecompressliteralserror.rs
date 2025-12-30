// Generated macro for DecompressLiteralsError (enum)
macro_rules! Depcrate_decoding_errorsDecompressLiteralsError {
() => {
// Module: crate::decoding::errors
// Provides: {"DecompressLiteralsError"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum DecompressLiteralsError { MissingCompressedSize , MissingNumStreams , GetBitsError (GetBitsError) , HuffmanTableError (HuffmanTableError) , HuffmanDecoderError (HuffmanDecoderError) , UninitializedHuffmanTable , MissingBytesForJumpHeader { got : usize } , MissingBytesForLiterals { got : usize , needed : usize } , ExtraPadding { skipped_bits : i32 } , BitstreamReadMismatch { read_til : isize , expected : isize } , DecodedLiteralCountMismatch { decoded : usize , expected : usize } , }
};
}
