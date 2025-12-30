// Generated macro for slice_assume_init (function)
macro_rules! Depcrate_io_read_bufslice_assume_init {
() => {
// Module: crate::io::read_buf
// Provides: {"slice_assume_init"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `slice` is fully initialized."] unsafe fn slice_assume_init (slice : & [MaybeUninit < u8 >]) -> & [u8] { unsafe { & * (slice as * const [MaybeUninit < u8 >] as * const [u8]) } }
};
}
