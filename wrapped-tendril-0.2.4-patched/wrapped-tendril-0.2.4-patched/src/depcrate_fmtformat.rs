// Generated macro for Format (trait)
macro_rules! Depcrate_fmtFormat {
() => {
// Module: crate::fmt
// Provides: {"Format"}
// Dependencies: {}
# [doc = " Trait for format marker types."] # [doc = ""] # [doc = " The type implementing this trait is usually not instantiated."] # [doc = " It's used with a phantom type parameter of `Tendril`."] pub unsafe trait Format { # [doc = " Check whether the buffer is valid for this format."] fn validate (buf : & [u8]) -> bool ; # [doc = " Check whether the buffer is valid for this format."] # [doc = ""] # [doc = " You may assume the buffer is a prefix of a valid buffer."] # [inline] fn validate_prefix (buf : & [u8]) -> bool { < Self as Format > :: validate (buf) } # [doc = " Check whether the buffer is valid for this format."] # [doc = ""] # [doc = " You may assume the buffer is a suffix of a valid buffer."] # [inline] fn validate_suffix (buf : & [u8]) -> bool { < Self as Format > :: validate (buf) } # [doc = " Check whether the buffer is valid for this format."] # [doc = ""] # [doc = " You may assume the buffer is a contiguous subsequence"] # [doc = " of a valid buffer, but not necessarily a prefix or"] # [doc = " a suffix."] # [inline] fn validate_subseq (buf : & [u8]) -> bool { < Self as Format > :: validate (buf) } # [doc = " Compute any fixup needed when concatenating buffers."] # [doc = ""] # [doc = " The default is to do nothing."] # [doc = ""] # [doc = " The function is `unsafe` because it may assume the input"] # [doc = " buffers are already valid for the format. Also, no"] # [doc = " bounds-checking is performed on the return value!"] # [inline (always)] unsafe fn fixup (_lhs : & [u8] , _rhs : & [u8]) -> imp :: Fixup { Default :: default () } }
};
}
