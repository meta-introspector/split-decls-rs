// Generated macro for IN4_IS_ADDR_UNSPECIFIED (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_UNSPECIFIED {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_UNSPECIFIED"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_UNSPECIFIED (a : & IN_ADDR) -> bool { unsafe { * a . S_un . S_addr () == IN4ADDR_ANY } }
};
}
