// Generated macro for IN4_IS_ADDR_LINKLOCAL (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_LINKLOCAL {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_LINKLOCAL"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_LINKLOCAL (a : & IN_ADDR) -> bool { unsafe { (* a . S_un . S_addr () & 0xffff) == 0xfea9 } }
};
}
