macro_rules! slice_assume_init {
    () => {
        # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `slice` is fully initialized."] unsafe fn slice_assume_init (slice : & [MaybeUninit < u8 >]) -> & [u8] { unsafe { & * (slice as * const [MaybeUninit < u8 >] as * const [u8]) } }
    };
}

slice_assume_init!();