macro_rules! deps {
    () => {
        BlockDecoder!();
        DecoderState!();
    };
}

macro_rules! new {
    () => {
        deps!();
        # [doc = " Create a new [BlockDecoder]."] pub fn new () -> BlockDecoder { BlockDecoder { internal_state : DecoderState :: ReadyToDecodeNextHeader , header_buffer : [0u8 ; 3] , } }
    };
}

new!()