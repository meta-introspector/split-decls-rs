// Generated macro for slice_assume_init_mut (function)
macro_rules! Depcrate_io_read_bufslice_assume_init_mut {
() => {
// Module: crate::io::read_buf
// Provides: {"slice_assume_init_mut"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that `slice` is fully initialized."] unsafe fn slice_assume_init_mut (slice : & mut [MaybeUninit < u8 >]) -> & mut [u8] { unsafe { & mut * (slice as * mut [MaybeUninit < u8 >] as * mut [u8]) } }
};
}
