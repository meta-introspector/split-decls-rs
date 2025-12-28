macro_rules! align_for_cmsghdr {
    () => {
        # [doc = " Return a slice of `buffer` starting at the first `cmsghdr` alignment"] # [doc = " boundary."] # [inline] fn align_for_cmsghdr (buffer : & mut [MaybeUninit < u8 >]) -> & mut [MaybeUninit < u8 >] { if buffer . is_empty () { return buffer ; } let align = align_of :: < c :: cmsghdr > () ; let addr = buffer . as_ptr () as usize ; let adjusted = (addr + (align - 1)) & align . wrapping_neg () ; & mut buffer [adjusted - addr ..] }
    };
}

align_for_cmsghdr!();