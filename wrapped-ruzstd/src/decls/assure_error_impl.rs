macro_rules! deps {
    () => {
        FrameDecoderError!();
        Error!();
    };
}

macro_rules! assure_error_impl {
    () => {
        deps!();
        # [cfg (all (test , feature = "std"))] # [allow (dead_code)] fn assure_error_impl () { use crate :: decoding :: errors :: FrameDecoderError ; let _err : & dyn std :: error :: Error = & FrameDecoderError :: NotYetInitialized ; }
    };
}

assure_error_impl!();