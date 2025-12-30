// Generated macro for impl_158 (impl)
macro_rules! Depcrate_decoding_streaming_decoderimpl_158 {
() => {
// Module: crate::decoding::streaming_decoder
// Provides: {"impl_158"}
// Dependencies: {}
impl < READ : Read , DEC : BorrowMut < FrameDecoder > > StreamingDecoder < READ , DEC > { pub fn new_with_decoder (mut source : READ , mut decoder : DEC ,) -> Result < StreamingDecoder < READ , DEC > , FrameDecoderError > { decoder . borrow_mut () . init (& mut source) ? ; Ok (StreamingDecoder { decoder , source }) } }
};
}
