// Generated macro for IN4_IS_ADDR_LOOPBACK (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_LOOPBACK {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_LOOPBACK"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_LOOPBACK (a : & IN_ADDR) -> bool { unsafe { a . S_un . S_un_b () . s_b1 == 0x7f } }
};
}
