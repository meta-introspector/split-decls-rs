// Generated macro for sys (module)
macro_rules! Depcrate_io_uringsys {
() => {
// Module: crate::io_uring
// Provides: {"sys"}
// Dependencies: {}
mod sys { pub (super) use linux_raw_sys :: io_uring :: * ; # [cfg (test)] pub (super) use { crate :: backend :: c :: iovec , linux_raw_sys :: general :: open_how , linux_raw_sys :: net :: msghdr , } ; }
};
}
