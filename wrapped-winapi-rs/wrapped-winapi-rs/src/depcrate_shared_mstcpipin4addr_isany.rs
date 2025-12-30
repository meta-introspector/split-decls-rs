// Generated macro for IN4ADDR_ISANY (function)
macro_rules! Depcrate_shared_mstcpipIN4ADDR_ISANY {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4ADDR_ISANY"}
// Dependencies: {}
# [inline] pub fn IN4ADDR_ISANY (a : & SOCKADDR_IN) -> bool { IN4_IS_ADDR_UNSPECIFIED (& a . sin_addr) }
};
}
