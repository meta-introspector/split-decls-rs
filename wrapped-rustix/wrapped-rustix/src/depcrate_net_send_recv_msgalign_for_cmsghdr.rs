// Generated macro for align_for_cmsghdr (function)
macro_rules! Depcrate_net_send_recv_msgalign_for_cmsghdr {
() => {
// Module: crate::net::send_recv::msg
// Provides: {"align_for_cmsghdr"}
// Dependencies: {}
# [doc = " Return a slice of `buffer` starting at the first `cmsghdr` alignment"] # [doc = " boundary."] # [inline] fn align_for_cmsghdr (buffer : & mut [MaybeUninit < u8 >]) -> & mut [MaybeUninit < u8 >] { if buffer . is_empty () { return buffer ; } let align = align_of :: < c :: cmsghdr > () ; let addr = buffer . as_ptr () as usize ; let adjusted = (addr + (align - 1)) & align . wrapping_neg () ; & mut buffer [adjusted - addr ..] }
};
}
