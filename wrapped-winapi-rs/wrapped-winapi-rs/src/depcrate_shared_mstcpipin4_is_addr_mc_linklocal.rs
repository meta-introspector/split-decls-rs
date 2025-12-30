// Generated macro for IN4_IS_ADDR_MC_LINKLOCAL (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_MC_LINKLOCAL {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_MC_LINKLOCAL"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_MC_LINKLOCAL (a : & IN_ADDR) -> bool { unsafe { (* a . S_un . S_addr () & 0xffffff) == 0xe0 } }
};
}
