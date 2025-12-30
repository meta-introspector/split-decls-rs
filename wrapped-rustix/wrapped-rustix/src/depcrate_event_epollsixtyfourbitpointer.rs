// Generated macro for SixtyFourBitPointer (struct)
macro_rules! Depcrate_event_epollSixtyFourBitPointer {
() => {
// Module: crate::event::epoll
// Provides: {"SixtyFourBitPointer"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Clone)] struct SixtyFourBitPointer { # [cfg (target_endian = "big")] # [cfg (target_pointer_width = "32")] _padding : u32 , pointer : * mut c_void , # [cfg (target_endian = "little")] # [cfg (target_pointer_width = "32")] _padding : u32 , }
};
}
