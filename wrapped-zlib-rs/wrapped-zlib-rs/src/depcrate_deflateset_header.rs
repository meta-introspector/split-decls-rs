// Generated macro for set_header (function)
macro_rules! Depcrate_deflateset_header {
() => {
// Module: crate::deflate
// Provides: {"set_header"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " The caller must guarantee:"] # [doc = ""] # [doc = " * If `head` is `Some`"] # [doc = "     - `head.extra` is `NULL` or is readable for at least `head.extra_len` bytes"] # [doc = "     - `head.name` is `NULL` or satisfies the requirements of [`core::ffi::CStr::from_ptr`]"] # [doc = "     - `head.comment` is `NULL` or satisfies the requirements of [`core::ffi::CStr::from_ptr`]"] pub unsafe fn set_header < 'a > (stream : & mut DeflateStream < 'a > , head : Option < & 'a mut gz_header > ,) -> ReturnCode { if stream . state . wrap != 2 { ReturnCode :: StreamError as _ } else { stream . state . gzhead = head ; ReturnCode :: Ok as _ } }
};
}
