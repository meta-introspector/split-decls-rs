// Generated macro for IN4_IS_ADDR_RFC1918 (function)
macro_rules! Depcrate_shared_mstcpipIN4_IS_ADDR_RFC1918 {
() => {
// Module: crate::shared::mstcpip
// Provides: {"IN4_IS_ADDR_RFC1918"}
// Dependencies: {}
# [inline] pub fn IN4_IS_ADDR_RFC1918 (a : & IN_ADDR) -> bool { let s_addr = unsafe { * a . S_un . S_addr () } ; ((s_addr & 0x00ff) == 0x0a) || ((s_addr & 0xf0ff) == 0x10ac) || ((s_addr & 0xffff) == 0xa8c0) }
};
}
