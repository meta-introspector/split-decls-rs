macro_rules! deps {
    () => {
        DecoderState!();
    };
}

macro_rules! BlockDecoder {
    () => {
        deps!();
        pub struct BlockDecoder { header_buffer : [u8 ; 3] , internal_state : DecoderState , }
    };
}

BlockDecoder!();