// Generated macro for new (function)
macro_rules! Depcrate_decoding_block_decodernew {
() => {
// Module: crate::decoding::block_decoder
// Provides: {"new"}
// Dependencies: {}
# [doc = " Create a new [BlockDecoder]."] pub fn new () -> BlockDecoder { BlockDecoder { internal_state : DecoderState :: ReadyToDecodeNextHeader , header_buffer : [0u8 ; 3] , } }
};
}
