// Generated macro for IN4_IS_ADDR_MC_ADMINLOCAL (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_MC_ADMINLOCAL {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_MC_ADMINLOCAL"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_MC_ADMINLOCAL (a : & IN_ADDR) -> bool { unsafe { (* a . S_un . S_addr () & 0xffff) == 0xffef } }
};
}
