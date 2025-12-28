macro_rules! TELL_ANY_CHECK {
    () => {
        # [doc = " A flag passed when initializing a decoder, causes `process` to return"] # [doc = " `Status::GetCheck` as soon as the integrity check is known."] pub const TELL_ANY_CHECK : u32 = lzma_sys :: LZMA_TELL_ANY_CHECK ;
    };
}

TELL_ANY_CHECK!()