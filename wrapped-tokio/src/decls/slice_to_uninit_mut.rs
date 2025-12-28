macro_rules! slice_to_uninit_mut {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `slice` is fully initialized"] # [doc = " and never writes uninitialized bytes to the returned slice."] unsafe fn slice_to_uninit_mut (slice : & mut [u8]) -> & mut [MaybeUninit < u8 >] { unsafe { & mut * (slice as * mut [u8] as * mut [MaybeUninit < u8 >]) } }
    };
}

slice_to_uninit_mut!();