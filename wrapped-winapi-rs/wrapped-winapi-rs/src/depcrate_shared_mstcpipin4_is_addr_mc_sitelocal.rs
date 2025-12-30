// Generated macro for IN4_IS_ADDR_MC_SITELOCAL (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_MC_SITELOCAL {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_MC_SITELOCAL"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_MC_SITELOCAL (a : & IN_ADDR) -> bool { let first = unsafe { (* a . S_un . S_addr () & 0xff) == 0xef } ; first && ! IN4_IS_ADDR_MC_ADMINLOCAL (a) }
};
}
