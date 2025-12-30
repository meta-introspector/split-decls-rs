// Generated macro for IN4_IS_UNALIGNED_ADDR_MULTICAST (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_UNALIGNED_ADDR_MULTICAST {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_UNALIGNED_ADDR_MULTICAST"}
// Dependencies: {}
# [inline] pub fn IN4_IS_UNALIGNED_ADDR_MULTICAST (a : & IN_ADDR) -> bool { IN4_MULTICAST (unsafe { * a . S_un . S_addr () as LONG }) }
};
}
