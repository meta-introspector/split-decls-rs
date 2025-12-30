// Generated macro for impl_147 (impl)
macro_rules! Depcrate_decoding_frame_decoderimpl_147 {
() => {
// Module: crate::decoding::frame_decoder
// Provides: {"impl_147"}
// Dependencies: {}
impl FrameDecoderState { pub fn new (source : impl Read) -> Result < FrameDecoderState , FrameDecoderError > { let (frame , header_size) = frame :: read_frame_header (source) ? ; let window_size = frame . window_size () ? ; Ok (FrameDecoderState { frame_header : frame , frame_finished : false , block_counter : 0 , decoder_scratch : DecoderScratch :: new (window_size as usize) , bytes_read_counter : u64 :: from (header_size) , check_sum : None , using_dict : None , }) } pub fn reset (& mut self , source : impl Read) -> Result < () , FrameDecoderError > { let (frame_header , header_size) = frame :: read_frame_header (source) ? ; let window_size = frame_header . window_size () ? ; if window_size > MAXIMUM_ALLOWED_WINDOW_SIZE { return Err (FrameDecoderError :: WindowSizeTooBig { requested : window_size , }) ; } self . frame_header = frame_header ; self . frame_finished = false ; self . block_counter = 0 ; self . decoder_scratch . reset (window_size as usize) ; self . bytes_read_counter = u64 :: from (header_size) ; self . check_sum = None ; self . using_dict = None ; Ok (()) } }
};
}
