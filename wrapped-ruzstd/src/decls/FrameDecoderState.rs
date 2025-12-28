macro_rules! deps {
    () => {
        DecoderScratch!();
        FrameHeader!();
    };
}

macro_rules! FrameDecoderState {
    () => {
        deps!();
        struct FrameDecoderState { pub frame_header : frame :: FrameHeader , decoder_scratch : DecoderScratch , frame_finished : bool , block_counter : usize , bytes_read_counter : u64 , check_sum : Option < u32 > , using_dict : Option < u32 > , }
    };
}

FrameDecoderState!()