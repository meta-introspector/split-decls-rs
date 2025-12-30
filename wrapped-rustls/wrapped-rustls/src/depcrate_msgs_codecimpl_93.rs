// Generated macro for impl_93 (impl)
macro_rules! Depcrate_msgs_codecimpl_93 {
() => {
// Module: crate::msgs::codec
// Provides: {"impl_93"}
// Dependencies: {}
impl Drop for LengthPrefixedBuffer < '_ > { # [doc = " Goes back and corrects the length previously inserted at the start of the structure."] fn drop (& mut self) { match self . size_len { ListLength :: NonZeroU8 { .. } => { let len = self . buf . len () - self . len_offset - 1 ; debug_assert ! (len <= 0xff) ; self . buf [self . len_offset] = len as u8 ; } ListLength :: U16 | ListLength :: NonZeroU16 { .. } => { let len = self . buf . len () - self . len_offset - 2 ; debug_assert ! (len <= 0xffff) ; let out : & mut [u8 ; 2] = (& mut self . buf [self . len_offset .. self . len_offset + 2]) . try_into () . unwrap () ; * out = u16 :: to_be_bytes (len as u16) ; } ListLength :: U24 { .. } | ListLength :: NonZeroU24 { .. } => { let len = self . buf . len () - self . len_offset - 3 ; debug_assert ! (len <= 0xff_ffff) ; let len_bytes = u32 :: to_be_bytes (len as u32) ; let out : & mut [u8 ; 3] = (& mut self . buf [self . len_offset .. self . len_offset + 3]) . try_into () . unwrap () ; out . copy_from_slice (& len_bytes [1 ..]) ; } } } }
};
}
