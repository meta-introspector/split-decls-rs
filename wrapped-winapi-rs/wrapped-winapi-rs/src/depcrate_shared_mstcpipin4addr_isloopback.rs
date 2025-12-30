// Generated macro for IN4ADDR_ISLOOPBACK (function)
macro_rules! Depcrate_shared_mstcpipIN4ADDR_ISLOOPBACK {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4ADDR_ISLOOPBACK"}
// Dependencies: {}
# [inline] pub fn IN4ADDR_ISLOOPBACK (a : & SOCKADDR_IN) -> bool { IN4_IS_ADDR_LOOPBACK (& a . sin_addr) }
};
}
