// Generated macro for FrameDecoderState (struct)
macro_rules! Depcrate_decoding_frame_decoderFrameDecoderState {
() => {
// Module: crate::decoding::frame_decoder
// Provides: {"FrameDecoderState"}
// Dependencies: {}
struct FrameDecoderState { pub frame_header : frame :: FrameHeader , decoder_scratch : DecoderScratch , frame_finished : bool , block_counter : usize , bytes_read_counter : u64 , check_sum : Option < u32 > , using_dict : Option < u32 > , }
};
}
