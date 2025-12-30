// Generated macro for impl_159 (impl)
macro_rules! Depcrate_decoding_streaming_decoderimpl_159 {
() => {
// Module: crate::decoding::streaming_decoder
// Provides: {"impl_159"}
// Dependencies: {}
impl < READ : Read > StreamingDecoder < READ , FrameDecoder > { pub fn new (mut source : READ ,) -> Result < StreamingDecoder < READ , FrameDecoder > , FrameDecoderError > { let mut decoder = FrameDecoder :: new () ; decoder . init (& mut source) ? ; Ok (StreamingDecoder { decoder , source }) } }
};
}
