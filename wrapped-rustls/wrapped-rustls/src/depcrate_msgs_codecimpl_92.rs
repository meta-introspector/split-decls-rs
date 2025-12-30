// Generated macro for impl_92 (impl)
macro_rules! Depcrate_msgs_codecimpl_92 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_92"}
// Dependencies: {}
impl < 'a > LengthPrefixedBuffer < 'a > { # [doc = " Inserts a dummy length into `buf`, and remembers where it went."] # [doc = ""] # [doc = " After this, the body of the length-delimited structure should be appended to `LengthPrefixedBuffer::buf`."] # [doc = " The length header is corrected in `LengthPrefixedBuffer::drop`."] pub (crate) fn new (size_len : ListLength , buf : & 'a mut Vec < u8 >) -> Self { let len_offset = buf . len () ; buf . extend (match size_len { ListLength :: NonZeroU8 { .. } => & [0xff] [..] , ListLength :: U16 | ListLength :: NonZeroU16 { .. } => & [0xff , 0xff] , ListLength :: U24 { .. } | ListLength :: NonZeroU24 { .. } => & [0xff , 0xff , 0xff] , }) ; Self { buf , len_offset , size_len , } } }
};
}
