// Generated macro for IN4_IS_ADDR_BROADCAST (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_BROADCAST {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_BROADCAST"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_BROADCAST (a : & IN_ADDR) -> bool { unsafe { * a . S_un . S_addr () == IN4ADDR_BROADCAST } }
};
}
