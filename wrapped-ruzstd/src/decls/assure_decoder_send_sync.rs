macro_rules! deps {
    () => {
        FrameDecoder!();
    };
}

macro_rules! assure_decoder_send_sync {
    () => {
        deps!();
        # [cfg (all (test , feature = "std"))] # [allow (dead_code)] fn assure_decoder_send_sync () { use crate :: decoding :: FrameDecoder ; let decoder = FrameDecoder :: new () ; std :: thread :: spawn (move | | { drop (decoder) ; }) ; }
    };
}

assure_decoder_send_sync!()