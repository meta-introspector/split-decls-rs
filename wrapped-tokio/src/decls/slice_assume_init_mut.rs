macro_rules! slice_assume_init_mut {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `slice` is fully initialized."] unsafe fn slice_assume_init_mut (slice : & mut [MaybeUninit < u8 >]) -> & mut [u8] { unsafe { & mut * (slice as * mut [MaybeUninit < u8 >] as * mut [u8]) } }
    };
}

slice_assume_init_mut!();